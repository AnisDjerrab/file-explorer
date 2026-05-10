const invoke = window.__TAURI__.core.invoke;

// this code creates the behavior with the hiding side bar
const sidebar = document.querySelector(".sidebar");
const main = document.querySelector(".main");
const file_grid = document.querySelector(".file_icons_grid");
const back = document.querySelector(".back");
const next = document.querySelector(".next");
const selection_box = document.getElementById("selection-box");
const path_section_elements = document.querySelector(".divided_path_area");
const right_scroll = document.getElementById("right_divided_path_area_button");
const left_scroll = document.getElementById("left_divided_path_area_button");

let currentPath = "";
let path_undo_redo_list = [];
let list_index = 0;

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
  selected_elements: [],
};

// now, communicate with rust to get the 'home' path
invoke("get_home_directory").then((home_dir) => {
  changePath(home_dir);
  currentPath = home_dir;
});

function changePath(newPath) {
  if (newPath != "Err : invalid path.") {
    document.querySelector(".path_area").value = newPath + "/";
    if (list_index >= path_undo_redo_list.length) {
      path_undo_redo_list.push(newPath);
    } else {
      path_undo_redo_list[list_index] = newPath;
    }
    file_grid.replaceChildren();
    file_grid.appendChild(selection_box);
    let divided_path = newPath.split("/");
    path_section_elements.replaceChildren();
    path_section_elements.appendChild(left_scroll);
    {
      const path_sec = document.createElement("button");
      path_sec.className = "path_section";
      const sys_svg_img = document.createElement("img");
      sys_svg_img.className = "svg";
      path_sec.appendChild(sys_svg_img);
      path_section_elements.appendChild(path_sec);
    }
    for (let i = 1; i < divided_path.length; i++) {
      const path_sec = document.createElement("button");
      path_sec.textContent = divided_path[i];
      path_sec.className = "path_section";
      path_section_elements.appendChild(path_sec);
    }
    path_section_elements.appendChild(right_scroll);
    // try to communicate with rust to get the files in home
    invoke("ls_dir", {
      path: newPath + "/",
      sortMethod: "A-Z",
      sortMethodDfs: "dfs",
    }).then((output) => {
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
            for (
              let i = 0;
              i < file_grid_content.selected_elements.length;
              i++
            ) {
              file_grid_content.selected_elements[i].classList.remove(
                "selected",
              );
            }
            file_grid_content.selected_elements = [];
          }
          file_grid_content.selected_elements.push(e.currentTarget);
          e.currentTarget.classList.add("selected");
        });
        file_icon.addEventListener("dblclick", async (e) => {
          if (output[2][i] == "f" || output[2][i] == "s") {
            let full_path = newPath + "/" + output[0][i];
            invoke("open_file_with_default_app", { path: full_path });
          } else {
            currentPath = newPath + "/" + output[0][i];
            list_index++;
            path_undo_redo_list.length = list_index;
            changePath(currentPath);
          }
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
      file_grid_content.selected_elements = [];
    });
  } else {
    document.querySelector(".path_area").value = newPath;
  }
}

let in_mouse_sel = false;
let initial_click_zone = {
  pos_x: 0,
  pos_y: 0,
};

// get an initial click
file_grid.addEventListener("mousedown", (e) => {
  initial_click_zone.pos_x = e.clientX;
  initial_click_zone.pos_y = e.clientY;
  selection_box.style.display = "none";
  selection_box.style.width = "0";
  selection_box.style.height = "0";
});

// get the click release, and conclude if it was an area selection or not
file_grid.addEventListener("mousemove", (e) => {
  if (
    initial_click_zone.pos_x != e.clientX &&
    initial_click_zone.pos_y != e.clientY &&
    e.buttons === 1
  ) {
    const gridRect = file_grid.getBoundingClientRect();
    let startX =
      initial_click_zone.pos_x - gridRect.left + file_grid.scrollLeft;
    let startY = initial_click_zone.pos_y - gridRect.top + file_grid.scrollTop;
    let currentX = e.clientX - gridRect.left + file_grid.scrollLeft;
    let currentY = e.clientY - gridRect.top + file_grid.scrollTop;
    let position_x = Math.min(startX, currentX);
    let position_y = Math.min(startY, currentY);
    let end_x = Math.max(startX, currentX);
    let end_y = Math.max(startY, currentY);
    selection_box.style.left = position_x + "px";
    selection_box.style.top = position_y + "px";
    selection_box.style.width = Math.abs(position_x - end_x) + "px";
    selection_box.style.height = Math.abs(position_y - end_y) + "px";
    selection_box.style.display = "block";
    in_mouse_sel = true;
    const selection_boxRect = selection_box.getBoundingClientRect();
    document.querySelectorAll(".file_icon").forEach((icon) => {
      const rect = icon.getBoundingClientRect();
      const overlaps =
        rect.left < selection_boxRect.right &&
        rect.right > selection_boxRect.left &&
        rect.top < selection_boxRect.bottom &&
        rect.bottom > selection_boxRect.top;
      if (overlaps && !icon.classList.contains("selected")) {
        icon.classList.add("selected");
        file_grid_content.selected_elements.push(icon);
      }
    });
  }
});

// if the user clicks on the bare file grid => deselect all
file_grid.addEventListener("mouseup", (e) => {
  if (
    e.target === file_grid ||
    (e.target.classList.contains("file_icons_grid") && in_mouse_sel == false)
  ) {
    for (let i = 0; i < file_grid_content.selected_elements.length; i++) {
      file_grid_content.selected_elements[i].classList.remove("selected");
    }
    file_grid_content.selected_elements = [];
  }
});

back.addEventListener("click", () => {
  if (list_index > 0) {
    list_index--;
    changePath(path_undo_redo_list[list_index]);
  }
});

next.addEventListener("click", () => {
  if (list_index < path_undo_redo_list.length - 1) {
    list_index++;
    changePath(path_undo_redo_list[list_index]);
  }
});
