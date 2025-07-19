const btn = document.getElementById("downloadBtn");
const status = document.getElementById("status");
const progressBar = document.getElementById("progress");
const container = document.getElementById("progressContainer");
btn.addEventListener("click", async () => {
  btn.disabled = true;
  container.style.display = "block";
  status.textContent = "Starting download...";
  try {
    const url = btn.getAttribute("data-url");
    const expectedHash = btn.getAttribute("data-sha");
    const resp = await fetch(url);
    const reader = resp.body.getReader();
    const contentLength = parseInt(resp.headers.get("Content-Length")) || 0;
    let received = 0;
    const chunks = [];
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      chunks.push(value);
      received += value.length;
      const percent = contentLength
        ? Math.floor((received / contentLength) * 100)
        : 0;
      progressBar.value = percent;
      status.textContent = `Downloaded ${percent}%`;
    }
    const buffer = new Uint8Array(received);
    let position = 0;
    chunks.forEach((chunk) => {
      buffer.set(chunk, position);
      position += chunk.length;
    });
    const hashBuffer = await crypto.subtle.digest("SHA-256", buffer);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    const hashHex = hashArray
      .map((b) => b.toString(16).padStart(2, "0"))
      .join("");
    if (hashHex !== expectedHash) {
      status.textContent = `Checksum mismatch! Expected ${expectedHash}, got ${hashHex}.`;
      progressBar.value = 0;
      return;
    }
    status.textContent = "Checksum verified, preparing file...";
    const blob = new Blob([buffer]);
    const link = document.createElement("a");
    link.href = URL.createObjectURL(blob);
    link.download = url.split("/").pop();
    document.body.appendChild(link);
    link.click();
    link.remove();
    status.textContent = "Download complete!";
  } catch (error) {
    status.textContent = "Error during download: " + error.message;
    console.error("Download error:", error);
  } finally {
    btn.disabled = false;
  }
});
window.addEventListener("load", () => btn.click());
