const API = "http://127.0.0.1:3000"; // match your PORT const

// Load existing domains on popup open
fetch(`${API}/config`)
  .then(res => res.json())
  .then(data => {
    textarea.value = data.domains.map(d => d.term).join("\n");
  });

document.getElementById("save").addEventListener("click", () => {
  const domains = textarea.value
    .split("\n")
    .map(d => d.trim())
    .filter(Boolean)
    .map(term => ({ term }));

  fetch(`${API}/config`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ domains })
  }).then(() => {
    status.textContent = "Saved!";
    setTimeout(() => (status.textContent = ""), 1500);
  });
});
