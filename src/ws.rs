use actix::{
    Actor, ActorContext, Addr, StreamHandler, Handler,
    AsyncContext, WrapFuture, ActorFutureExt, fut,
    ContextFutureSpawner, Running
};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;
use std::time::{Duration, Instant};
use crate::{
    lobby::Lobby,
    message::{ClientActorMessage, WsMessage, Connect, Disconnect}
};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Define HTTP actor
#[derive(Debug)]
pub struct ChessWebSocket {
    id: Uuid,
    room: Uuid,
    hb: Instant,
    lobby_addr: Addr<Lobby>
}

impl ChessWebSocket {
    /// Create new Chess Websocket instance
    pub fn new(room: Uuid, lobby: Addr<Lobby>) -> ChessWebSocket {
        ChessWebSocket {
            id: Uuid::new_v4(),
            room,
            hb: Instant::now(),
            lobby_addr: lobby
        }
    }
}

impl Actor for ChessWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        let connect = Connect {
            addr: addr.recipient(),
            lobby_id: self.room,
            self_id: self.id,
        };

        self.lobby_addr
            .send(connect)
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.lobby_addr.do_send(Disconnect { id: self.id, room_id: self.room });
        Running::Stop
    }
}

impl ChessWebSocket {
    /// Heartbeat for checking the websocket connection
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting failed heartbeat");
                let disconnect = Disconnect { id: act.id, room_id: act.room };
                act.lobby_addr.do_send(disconnect);
                ctx.stop();
                return;
            }
            ctx.ping(b"hi");
        });
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChessWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => self.hb = Instant::now(),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => ctx.stop(),
            Ok(ws::Message::Nop) => (),
            Ok(ws::Message::Text(s)) => self.lobby_addr.do_send(
                ClientActorMessage {
                    id: self.id,
                    msg: s.to_string(),
                    room_id: self.room
                }
            ),
            Err(e) => panic!("{}", e),
        }
    }
}

impl Handler<WsMessage> for ChessWebSocket {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// Start the websocket connection
#[get("/ws/{room_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: web::Payload,
    room_id: web::Path<Uuid>,
    data_lobby_addr: web::Data<Addr<Lobby>>
) -> Result<HttpResponse, Error> {
    let chess_ws = ChessWebSocket::new(
        room_id.into_inner(),
        data_lobby_addr.get_ref().clone()
    );
    let resp = ws::start(chess_ws, &req, stream)?;
    Ok(resp)
}
