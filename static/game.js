let self_uuid = document.getElementById('game-id').innerText;
let uri = 'ws://' + window.location.host + '/ws/' + self_uuid;
let ws = new WebSocket(uri);
let session_id = "";

let ss = [];
let color = "";

let chess = {
  board: [],
  turn: 0,
  status: '',
  history: [],
  captured_black: [],
  captured_white: [],
  gameOver: false,
  possible_moves: [],
  yourTurn: false
};

let gameState = new Proxy(chess, {
  set: (target, prop, val) => {
    if (prop == 'board') {
      renderBoard(val);
      ss = [];

    } else if (prop == 'captured_white') {
      let capturedContainer = document.getElementById(`chess-captured-white`);
      if (val[0] !== "") {
          capturedContainer.innerHTML = "";
          val.map(piece => capturedContainer.append(createChessPiece(piece)));
      }

    } else if (prop == 'captured_black') {
      let capturedContainer = document.getElementById(`chess-captured-black`);
      if (val[0] !== "") {
          capturedContainer.innerHTML = "";
          val.map(piece => capturedContainer.append(createChessPiece(piece)));
      }

    } else if (prop == 'possible_moves') {
      if (target[prop][0] !== '')
        target[prop].map(pm => document.getElementById(pm).classList.remove("path"));

      if (val[0] !== '')
        val.map(pm => document.getElementById(pm).classList.add("path"));
    }

    // update the actual variable
    target[prop] = val;
    updateStatus();
  },
});

document.addEventListener('DOMContentLoaded', () => {
  ws.onopen = (_event) => {
    ws.send("\\get_color");
    ws.send("\\get_game_stat");
    ws.send("\\get_board");
    ws.send("\\get_status");
  };

  ws.onmessage = (event) => {
    let msg = event.data;
    // console.log(msg);

    if (msg.startsWith("board:")) {
      gameState.turn += 1;
      let devided = (color == "white" || color == "") ? 1 : 0;
      gameState.yourTurn = ((gameState.turn % 2) == devided) ? true : false;
      gameState.board = msg.split("\n")
        .splice(1, 8)
        .map((row) => row.trim().split(" "));

    } else if (msg.startsWith("promoted")) {
      gameState.board = msg.split("\n")
        .splice(1, 8)
        .map((row) => row.trim().split(" "));

    } else if (msg.startsWith("game stat")) {
      let gameStatus = msg.split("\n")[0].split(": ")[1];
      gameState.status = gameStatus;
      if (gameStatus == "game over") { gameState.gameOver = true }

      let stat = msg.split("\n").splice(1);
      gameState.turn = parseInt(stat[0].split(": ")[1]);
      updateStatus();

      let moveHistory = stat.splice(2, gameState.turn-1);
      if (moveHistory[0] !== "") {
        gameState.history = moveHistory;
        let historyContainer = document.getElementById("chess-move-history");
        moveHistory.forEach((his, idx) => {
          let pieceMove = document.createElement("div");
          pieceMove.innerText = `${idx+1}. ${his}`;
          historyContainer.append(pieceMove);
        })
      }

    } else if (msg.startsWith("status:")) {
      let status = msg.split(": ")[1];
      gameState.status = status;
      updateStatus()

    } else if (msg.startsWith("color:")) {
        color = msg.split(": ")[1];

    } else if (msg.startsWith("your session_id")) {
      session_id = msg.split(" ")[3];

    } else if (msg.startsWith("captured_white")) {
      gameState.captured_white = msg.split(" ").splice(1);

    } else if (msg.startsWith("captured_black")) {
      gameState.captured_black = msg.split(" ").splice(1);

    } else if (msg.startsWith("history")) {
      let moveHistory = msg.split(": ").splice(1)[0];
      gameState.history.push(moveHistory);
      // Update Chess Move History
      let historyContainer = document.getElementById("chess-move-history");
      let pieceMoves = document.createElement("div");
      pieceMoves.innerText = `${gameState.turn}. ${moveHistory}`;
      historyContainer.append(pieceMoves);
      if (msg.includes("Checkmate") || msg.includes("Draw")) {
        gameState.gameOver = true;
      }

    } else if (msg.startsWith("possible moves")) {
      let [pos, posMove] = msg.split("\n").splice(1);
      let vpm = posMove.split(" ");
      if (vpm[0] !== "") {
        ss.push(pos);
        document.getElementById(pos).classList.add("selected");
      }
      gameState.possible_moves = vpm;

    } else if (msg.startsWith("Error")) {
      alert(msg);
      ss = [ss[0]];

    }

  };
});

/**
 * Convert tuple (row, col) into chess notation
 */
function invert(row, col) {
  const alphabet = 'abcdefgh';
  return alphabet[col] + (row+1)
}

/**
 * Render the chess board
 */
function renderBoard(data) {
  let app = document.getElementById("chess-game");
  app.innerHTML = '';
  let pieceData = (color == "white") ? data : data.reverse();
  pieceData.forEach((row, y) => {
    if (color == "white") {
      row.forEach((cell, x) => {
        let cellColor = ((x+y)%2 == 0) ? "lightgrey" : "darkgrey";
        createChessElement(app, cell, x, row, y, cellColor);
      });
    } else {
      row.reverse().forEach((cell, x) => {
        let cellColor = ((x+y)%2 == 0) ? "lightgrey" : "darkgrey";
        createChessElement(app, cell, x, row, y, cellColor);
      });
    }
  });

  if (color == "black") {
    let boardContainer = document.getElementsByClassName("board-container")[0];
    boardContainer.style.flexDirection = "column-reverse";

    document.querySelectorAll(".label-before")
      .forEach((square) => {
        square.style.setProperty('--before-left', 'auto');
        square.style.setProperty('--before-right', '3px');
      });

    document.querySelectorAll(".label-after")
      .forEach((square) => {
        square.style.setProperty('--after-bottom', 'auto');
        square.style.setProperty('--after-top', '0');
      });
  }

  if (promotable(ss[1])) {
    promoteLevelSelector(ss[1]);
    ss = [ss[1]];
  }

  ws.send("\\get_captured white");
  ws.send("\\get_captured black");

  if (gameState.gameOver) {
    let gameOverDiv = document.createElement("div");
    gameOverDiv.innerText = "Game Over";
    document.getElementById("chess-move-history").append(gameOverDiv);
  }
}

/**
 * Update status container with 'White Turn', 'Black Turn', or 'Game Over'
 */
function updateStatus() {
  let statusContainer = document.getElementById("chess-status-container");
  let statusText = "";

  if (gameState.gameOver) {
    statusText = "Game Over"
  } else {
    statusText = ((gameState.turn % 2) == 0) ? "Black Turn" : "White Turn";
  }

  if (gameState.status == "waiting") statusText += " (Waiting Other Player)";
  statusContainer.innerText = statusText;
}

/**
 * Create HTMLElement from chess icon
 * @param {string} cell chess icon
 * @returns HTMLElement
 */
function createChessPiece(cell) {
  let childElm = document.createElement("i");
  switch (cell) {
    case "♟":
      childElm.classList.add("fas", "fa-chess-pawn", "chess-piece-black");
      break;
    case "♜":
      childElm.classList.add("fas", "fa-chess-rook", "chess-piece-black");
      break;
    case "♞":
      childElm.classList.add("fas", "fa-chess-knight", "chess-piece-black");
      break;
    case "♝":
      childElm.classList.add("fas", "fa-chess-bishop", "chess-piece-black");
      break;
    case "♛":
      childElm.classList.add("fas", "fa-chess-queen", "chess-piece-black");
      break;
    case "♚":
      childElm.classList.add("fas", "fa-chess-king", "chess-piece-black");
      break;
    case "♙":
      childElm.classList.add("fas", "fa-chess-pawn", "chess-piece-white");
      break;
    case "♖":
      childElm.classList.add("fas", "fa-chess-rook", "chess-piece-white");
      break;
    case "♘":
      childElm.classList.add("fas", "fa-chess-knight", "chess-piece-white");
      break;
    case "♗":
      childElm.classList.add("fas", "fa-chess-bishop", "chess-piece-white");
      break;
    case "♕":
      childElm.classList.add("fas", "fa-chess-queen", "chess-piece-white");
      break;
    case "♔":
      childElm.classList.add("fas", "fa-chess-king", "chess-piece-white");
      break;
    default:
      break;
  };
  return childElm;
}

/**
 * Create chess square and it's event listener
 * @param {string} app Game container
 * @param {string} cell Location in chess notation, e.g. "a1"
 * @param {int} x index of row
 * @param {Array} row
 * @param {int} y index of column
 * @param {string} cellColor
 */
function createChessElement(app, cell, x, _row, y, cellColor) {
  let elm = document.createElement("a");
  let kolom = (color == "white") ? 7-y : y;
  let baris = (color == "white") ? x : 7-x;
  let pos = invert(kolom, baris);
  elm.setAttribute("id", pos);
  elm.classList.add("square", "btn");
  elm.style.backgroundColor = cellColor;

  if (pos.includes('a')) {
    let labelBefore = document.createElement('span');
    labelBefore.classList.add('label-before');
    labelBefore.innerText = pos[1];
    elm.append(labelBefore);
  }

  if (pos.includes('1')) {
    let labelAfter = document.createElement('span');
    labelAfter.classList.add('label-after');
    labelAfter.innerText = pos[0].toUpperCase();
    elm.append(labelAfter);
  }

  let childElm = createChessPiece(cell);
  elm.append(childElm);
  if ((!gameState.gameOver) && gameState.yourTurn) {
    elm.addEventListener("click", e => squareClickHandler(e, elm, childElm, pos));
  }
  app.append(elm);
}

/**
 * Clicked square event handler
 */
function squareClickHandler(event, elm, childElm, pos) {
  event.preventDefault();

  if (!childElm.className.match(/chess-piece/) && ss.length == 0) return false;

  if (childElm.className.match(/chess-piece-white/) && gameState.turn % 2 === 0 && ss.length === 0) {
    console.log("Can't move this piece, Black turn.");
    return false;
  }

  if (childElm.className.match(/chess-piece-black/) && gameState.turn % 2 === 1 && ss.length === 0) {
    console.log("Can't move this piece, White turn.");
    return false;
  }

  if (ss.includes(pos)) {
    ss.splice(ss.indexOf(pos), 1);
    elm.classList.remove("selected");
    gameState.possible_moves
      .map(pm => document.getElementById(pm).classList.remove("path"));

  } else if (ss.length == 0) {
    ws.send(`\\get_possible_moves ${pos}`);

  } else if (ss.length == 1) {
    if (gameState.possible_moves.includes(pos)) {
      ss.push(pos);
      ws.send(`\\move ${ss[0]} ${pos}`);

    } else {
      let king = document.getElementById(ss[0]).querySelector('i');
      let rook = document.getElementById(pos).querySelector('i');
      if (king.className.match(/fa-chess-king/) && rook.className.match(/fa-chess-rook/)) {
        ws.send(`\\castling ${ss[0]} ${pos}`);
      }
    }
  }
}

/**
 * Check if the pos is promotable or not
 * @param {string} pos position in the chess board to check
 * @returns Boolean
 */
function promotable(pos) {
  let piece = document.getElementById(pos);
  if (piece == null) return false;
  let isPawn = piece.querySelector('i').className.match(/fa-chess-pawn/);
  if (!isPawn) return false;
  if (!pos.includes("1") && !pos.includes("8")) return false;
  return true;
}

/**
 * Create level selector for pawn promotion
 * @param {string} pos position in the chess board
 */
function promoteLevelSelector(pos) {
  let pawn = document.getElementById(pos);
  let isPawn = pawn.querySelector('i').className.match(/chess-piece-white/);
  let color = (isPawn) ? "chess-piece-white" : "chess-piece-black";
  let options = ["queen", "bishop", "knight", "rook"];
  let optionContainer = document.createElement("div");

  optionContainer.setAttribute("class", "promotion-level-selector");
  options.map((opt) => {
    let child = document.createElement("a");
    child.setAttribute("id", opt);
    let grandChild = document.createElement("i");
    grandChild.setAttribute("class", `fas fa-chess-${opt} ${color}`);
    child.append(grandChild);
    optionContainer.append(child);
    child.addEventListener('click', (event) => {
      event.preventDefault();
      ws.send(`\\promote ${pos} ${opt}`);
    })
  });
  pawn.append(optionContainer);
}
