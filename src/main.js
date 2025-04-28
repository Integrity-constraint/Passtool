const { invoke } = window.__TAURI__.core;

const appWindow = invoke;
document
  .getElementById('titlebar-minimize')
  ?.addEventListener('click', () => appWindow.minimize());
document
  .getElementById('titlebar-maximize')
  ?.addEventListener('click', () => appWindow.toggleMaximize());
document
  .getElementById('titlebar-close')
  ?.addEventListener('click', () => appWindow.close());

  document.querySelector('.expander-button').addEventListener('click', function() {
    this.parentElement.classList.toggle('active');
  });

document.getElementById('generate-btn').addEventListener('click', async () => {
  const params = {
      length: parseInt(document.getElementById('length').value),
      uppercase: document.getElementById('uppercase').checked,
      lowercase: document.getElementById('lowercase').checked,
      numbers: document.getElementById('numbers').checked,
      symbols: document.getElementById('symbols').checked,
  };

  try {
      const password = await invoke('generate_password', { params });
      document.getElementById('password').textContent = password;
  } catch (err) {
      console.error("Generation error:", err);
      document.getElementById('password').textContent = "Error: " + err;
  }
});

document.getElementById('save-btn').addEventListener('click', async () => {
    try {
        const password_to_save = await invoke('get_last_password_copy');
        if (!password_to_save) {
            throw new Error("Нет пароля для сохранения");
        }
        
        document.getElementById('password-save').textContent = password_to_save;
        await invoke('save_to_file');
        alert("Пароль успешно сохранён!");
    } catch (err) {
        console.error("Ошибка сохранения:", err);
        document.getElementById('password-save').textContent = "Ошибка: " + err;
    }
});
  

document.getElementById('saveToList-btn').addEventListener('click', async () => {
    try {
        const password_to_save = await invoke('get_last_password_copy');
        if (!password_to_save) {
            throw new Error("Нет пароля для сохранения");
        }
        
        document.getElementById('password-save').textContent = password_to_save;
        await invoke('save_to_file');
        alert("Пароль успешно сохранён!");
    } catch (err) {
        console.error("Ошибка сохранения:", err);
        document.getElementById('password-save').textContent = "Ошибка: " + err;
    }
});

const drawer = document.getElementById('drawer');
const drawerOverlay = document.getElementById('drawer-overlay');
const openDrawerBtn = document.getElementById('drawer-button');
const closeDrawerBtn = document.getElementById('close-drawer');
const body = document.body;

openDrawerBtn.addEventListener('click', () => {
    drawer.classList.add('open');
    drawerOverlay.classList.add('open');
    body.classList.add('drawer-open');
});

closeDrawerBtn.addEventListener('click', () => {
    drawer.classList.remove('open');
    drawerOverlay.classList.remove('open');
    body.classList.remove('drawer-open');
});

drawerOverlay.addEventListener('click', () => {
    drawer.classList.remove('open');
    drawerOverlay.classList.remove('open');
    body.classList.remove('drawer-open');
});


document.getElementById('btn-bd-create').addEventListener('click', async () => {
    try {
        await invoke('create_bd');
    
        alert("БД успешно инициализирована!");
    } catch (err) {
        alert("Ошибка инициализации", err);
      
    }
});

document.getElementById('btn-crud-save').addEventListener('click', async () => {
    document.querySelectorAll("tbody tr").forEach(async (row) => {
        const cells = row.querySelectorAll("td");
        const resource = cells[1].innerText;
        const password = cells[2].innerText;

        await invoke("add_entry", { resource, password });
    });

    alert("Данные сохранены в SQLite!");
});



