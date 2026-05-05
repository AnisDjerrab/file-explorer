const invoke = window.__TAURI__.core.invoke;

// this code creates the behavior with the hiding side bar
const sidebar = document.querySelector(".sidebar");
const main = document.querySelector(".main");

function openSidebar() {
  sidebar.classList.add("open");
  main.classList.add("dim");
}

function closeSidebar() {
  sidebar.classList.remove("open");
  main.classList.remove("dim");
} 

document.addEventListener("click", (e) => {
  const width = window.innerWidth;

  if (e.clientX < 50) {
    openSidebar();
  }
});

document.addEventListener("click", (e) => {
  if (e.clientX > 250) {
    closeSidebar();
  }
});

// get the file grid element
const file_grid = document.querySelector(".file_icons_grid");

// now, communicate with rust to get the 'home' path
invoke('get_home_directory').then((home_dir) => document.querySelector(".path_area").value = home_dir + "/");
// try to communicate with rust to get the files in home
invoke('ls_dir', {path : "/home/anis/Documents/OpenGL-apps/", sortMethod : "A-Z", sortMethodDfs : "dfs"}).then((output) => {
  for (let i = 0; i < output[0].length / 2; i++) {
    const file_icon = document.createElement("div");
    file_icon.className = "file_icon";
    const img = document.createElement("img");
    img.src = output[0][i + output[0].length / 2];
    const icon_content = document.createElement("div");
    icon_content.textContent = output[0][i];
    file_icon.appendChild(img);
    file_icon.appendChild(icon_content);
    file_grid.appendChild(file_icon);
  }
});

