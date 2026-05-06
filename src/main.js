const invoke = window.__TAURI__.core.invoke;

// this code creates the behavior with the hiding side bar
const sidebar = document.querySelector(".sidebar");
const main = document.querySelector(".main");
const file_grid = document.querySelector(".file_icons_grid");

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

const file_grid_content = {
  files_paths: [],
  files_icons_paths: [],
  files_status: [],
  files_divs: [],
  selected_elements: []
}

// now, communicate with rust to get the 'home' path
invoke('get_home_directory').then((home_dir) => changePath(home_dir));

function changePath(newPath) {
  document.querySelector(".path_area").value = newPath + "/"
 // try to communicate with rust to get the files in home
  invoke('ls_dir', {path : newPath + "/", sortMethod : "A-Z", sortMethodDfs : "dfs"}).then((output) => {
    // get the file grid element
    let new_items = [];
    for (let i = 0; i < output[0].length; i++) {
      const file_icon = document.createElement("div");
      file_icon.className = "file_icon";
      const img = document.createElement("img");
      img.src = output[1][i];
      const icon_content = document.createElement("div");
      icon_content.textContent = output[0][i];
      new_items.push(file_icon);
      file_icon.addEventListener("click", (e) => {
        if (!e.ctrlKey && !e.metaKey) {
          for (let i = 0; i < file_grid_content.selected_elements.length; i++) {
            file_grid_content.selected_elements[i].classList.remove("selected");
          }
          file_grid_content.selected_elements = [];
        } 
        file_grid_content.selected_elements.push(e.currentTarget);
        e.currentTarget.classList.add("selected");
      });
      file_icon.addEventListener("contextmenu", (e) => {
        if (!e.currentTarget.classList.contains("selected")) {
          e.preventDefault();
          file_grid_content.selected_elements.push(e.currentTarget);
          e.currentTarget.classList.add("selected");  
        }
      });
      file_icon.appendChild(img);
      file_icon.appendChild(icon_content);
      file_grid.appendChild(file_icon);
    }
    file_grid_content.files_paths = output[0];
    file_grid_content.files_icons_paths = output[1];
    file_grid_content.files_status = output[2];
    file_grid_content.files_divs = new_items;
  });
}

let in_mouse_sel = false;
let initial_click_zone = {
  pos_x: 0,
  pos_y: 0
}

// get an initial click
file_grid.addEventListener("mousedown", (e) => {
  initial_click_zone.pos_x = e.clientX;
  initial_click_zone.pos_y = e.clientY;
  const box = document.getElementById("selection-box");
  const new_box = document.createElement("div");
  new_box.id = "selection-box";
  box.replaceWith(new_box);
});

// get the click release, and conclude if it was an area selection or not
file_grid.addEventListener("mousemove", (e) => {
  if (initial_click_zone.pos_x != e.clientX && initial_click_zone.pos_y != e.clientY && e.buttons === 1) {
    const box = document.getElementById("selection-box");
     const gridRect = file_grid.getBoundingClientRect();
    let startX = initial_click_zone.pos_x - gridRect.left;
    let startY = initial_click_zone.pos_y - gridRect.top;
    let currentX = e.clientX - gridRect.left;
    let currentY = e.clientY - gridRect.top;
    let position_x = Math.min(startX, currentX);
    let position_y = Math.min(startY, currentY);
    let end_x = Math.max(startX, currentX);
    let end_y = Math.max(startY, currentY);
    box.style.left = position_x + "px";
    box.style.top = position_y + "px";
    box.style.width = Math.abs(position_x - end_x) + "px";
    box.style.height = Math.abs(position_y - end_y) + "px";
    box.style.display = "block";
    in_mouse_sel = true;
    const boxRect = box.getBoundingClientRect();
    document.querySelectorAll(".file_icon").forEach(icon => {
      const rect = icon.getBoundingClientRect();
      const overlaps =
        rect.left < boxRect.right &&
        rect.right > boxRect.left &&
        rect.top < boxRect.bottom &&
        rect.bottom > boxRect.top;
      if (overlaps && !icon.classList.contains("selected")) {
        icon.classList.add("selected");
        file_grid_content.selected_elements.push(icon);
      }
    });
  }
});

// if the user clicks on the bare file grid => deselect all
file_grid.addEventListener("mouseup", (e) => {
  if (e.target === file_grid || e.target.classList.contains("file_icons_grid") && in_mouse_sel == false) {
    for (let i = 0; i < file_grid_content.selected_elements.length; i++) {
      file_grid_content.selected_elements[i].classList.remove("selected");
    }
    file_grid_content.selected_elements = [];
  }
});