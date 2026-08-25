(() => {
const button = document.getElementById("downloadBtn");
const status = document.getElementById("status");
const progressBar = document.getElementById("progress");
const progressContainer = document.getElementById("progressContainer");

function setStatus(message, state = "working") {
  status.textContent = message;
  status.dataset.state = state;
}

function downloadFilename(url) {
  try {
    const pathname = new URL(url, window.location.href).pathname;
    return decodeURIComponent(pathname.split("/").filter(Boolean).pop()) || "download";
  } catch {
    return "download";
  }
}

async function startDownload() {
  const url = button?.dataset.url;
  const expectedHash = button?.dataset.sha?.toLowerCase();
  if (!button || !status || !progressBar || !progressContainer || !url || !expectedHash) {
    return;
  }

  button.disabled = true;
  button.textContent = "Downloading…";
  progressContainer.hidden = false;
  progressBar.removeAttribute("value");
  setStatus("Connecting to the download source…");

  try {
    if (!window.crypto?.subtle) {
      throw new Error("Secure checksum verification is not available in this browser.");
    }

    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`The download server returned HTTP ${response.status}.`);
    }

    const contentLength = Number(response.headers.get("Content-Length")) || 0;
    const chunks = [];
    let received = 0;

    if (response.body) {
      const reader = response.body.getReader();
      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        chunks.push(value);
        received += value.byteLength;

        if (contentLength > 0) {
          const percent = Math.min(100, Math.round((received / contentLength) * 100));
          progressBar.value = percent;
          setStatus(`Downloading… ${percent}%`);
        } else {
          setStatus(`Downloaded ${(received / 1024 / 1024).toFixed(1)} MB…`);
        }
      }
    } else {
      const data = new Uint8Array(await response.arrayBuffer());
      chunks.push(data);
      received = data.byteLength;
    }

    const buffer = new Uint8Array(received);
    let position = 0;
    for (const chunk of chunks) {
      buffer.set(chunk, position);
      position += chunk.byteLength;
    }

    setStatus("Verifying SHA-256 checksum…");
    const hashBuffer = await crypto.subtle.digest("SHA-256", buffer);
    const actualHash = Array.from(new Uint8Array(hashBuffer), (byte) =>
      byte.toString(16).padStart(2, "0"),
    ).join("");

    if (actualHash !== expectedHash) {
      throw new Error("Checksum verification failed. The file was not saved.");
    }

    const objectUrl = URL.createObjectURL(new Blob([buffer]));
    const link = document.createElement("a");
    link.href = objectUrl;
    link.download = downloadFilename(url);
    document.body.appendChild(link);
    link.click();
    link.remove();
    window.setTimeout(() => URL.revokeObjectURL(objectUrl), 0);

    progressBar.value = 100;
    setStatus("Verified. Your download is ready.", "success");
    button.textContent = "Download again";
  } catch (error) {
    progressBar.value = 0;
    setStatus(error instanceof Error ? error.message : "The download failed.", "error");
    button.textContent = "Try again";
  } finally {
    button.disabled = false;
  }
}

button?.addEventListener("click", startDownload);
})();
