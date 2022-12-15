function str2uuid(uuid) {
  let result = uuid.substring(0, 8) + '-' + uuid.substring(8, 12) + '-' + uuid.substring(12, 16) + '-' + uuid.substring(16, 20) + '-' + uuid.substring(20);
  return result;
}

let self_uuid = document.getElementById("game-id").innerText;
console.log(self_uuid);

let joinBtn = document.getElementById("button-join");
joinBtn.addEventListener('click', (_event) => {
  let targetId = document.getElementById('input-game-id').value;
  window.location.href = '/game/' + targetId;
})
