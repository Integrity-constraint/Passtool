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
