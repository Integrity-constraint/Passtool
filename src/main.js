const { invoke } = window.__TAURI__.core;


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
  