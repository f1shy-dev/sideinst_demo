const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

const log = (msg) => {
  const logEl = document.querySelector("#log-window");
  logEl.innerHTML += `${new Date().toLocaleTimeString()} ${msg}<br />`;
};

(async () => {
  document.querySelector("#get-devices").addEventListener("click", async () => {
    log(await invoke("get_devices"));
  });

  document.querySelector("#clear-log").addEventListener("click", async () => {
    document.querySelector("#log-window").innerHTML = "";
  });
})();
