document.addEventListener("DOMContentLoaded", function () {
  const encryptButton = document.getElementById("encrypt-button");
  const messageForm = document.getElementById("message-form");
  const encryptionResult = document.getElementById("encryption-result");
  const newMessageButton = document.getElementById("new-message");

  encryptButton.addEventListener("click", async function () {
    const messageText = document.getElementById("message").value;
    if (!messageText) {
      alert("Please enter a message to encrypt.");
      return;
    }

    try {
      // Generate a random encryption key
      const key = await generateEncryptionKey();

      // Convert the key to a format that can be downloaded
      const exportedKey = await window.crypto.subtle.exportKey("raw", key);
      const keyAsString = arrayBufferToBase64(exportedKey);

      // Generate a random IV
      const iv = window.crypto.getRandomValues(new Uint8Array(12));
      const ivAsString = arrayBufferToBase64(iv);

      // Encrypt the message
      const encryptedData = await encryptMessage(messageText, key, iv);
      const encryptedBase64 = arrayBufferToBase64(encryptedData);

      // Download the key file
      downloadKey(keyAsString);

      // Send the encrypted message to the server
      await sendEncryptedMessage(encryptedBase64, ivAsString);

      // Show success message
      messageForm.style.display = "none";
      encryptionResult.style.display = "block";
    } catch (error) {
      console.error("Encryption failed:", error);
      alert("Encryption failed: " + error.message);
    }
  });

  newMessageButton.addEventListener("click", function () {
    document.getElementById("message").value = "";
    messageForm.style.display = "block";
    encryptionResult.style.display = "none";
  });

  async function generateEncryptionKey() {
    return window.crypto.subtle.generateKey(
      {
        name: "AES-GCM",
        length: 256,
      },
      true,
      ["encrypt", "decrypt"],
    );
  }

  async function encryptMessage(message, key, iv) {
    const encoder = new TextEncoder();
    const encodedMessage = encoder.encode(message);

    return window.crypto.subtle.encrypt(
      {
        name: "AES-GCM",
        iv: iv,
      },
      key,
      encodedMessage,
    );
  }

  function arrayBufferToBase64(buffer) {
    const bytes = new Uint8Array(buffer);
    let binary = "";
    for (let i = 0; i < bytes.byteLength; i++) {
      binary += String.fromCharCode(bytes[i]);
    }
    return btoa(binary);
  }

  function downloadKey(keyString) {
    const element = document.createElement("a");
    // Create a text blob with the base64 string
    const blob = new Blob([keyString], { type: "text/plain" });
    const url = URL.createObjectURL(blob);

    element.setAttribute("href", url);
    element.setAttribute(
      "download",
      "encryption-key-" + new Date().getTime() + ".txt",
    );
    element.style.display = "none";
    document.body.appendChild(element);
    element.click();

    // Clean up
    document.body.removeChild(element);
    URL.revokeObjectURL(url);
  }

  async function sendEncryptedMessage(encryptedData, iv) {
    // Display the encrypted message
    const encryptedDisplay = document.getElementById(
      "encrypted-message-display",
    );
    encryptedDisplay.textContent = JSON.stringify({
      encrypted_message: encryptedData,
      iv: iv,
    });

    const response = await fetch("/api/messages", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        encrypted_message: encryptedData,
        iv: iv,
      }),
    });

    if (!response.ok) {
      throw new Error("Failed to save message");
    }

    return await response.json();
  }
});

document
  .getElementById("copy-encrypted")
  .addEventListener("click", function () {
    const text = document.getElementById(
      "encrypted-message-display",
    ).textContent;
    navigator.clipboard.writeText(text).then(
      function () {
        alert("Encrypted message copied to clipboard!");
      },
      function (err) {
        console.error("Could not copy text: ", err);
      },
    );
  });
