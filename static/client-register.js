async function handleRegisterClick(e) {
  e.preventDefault();

  const username = document.getElementById("username").value;
  const aliases = document.getElementById("aliases").value;

  Toast.info("Starting registration process...", "Registration");

  const p = new Passwordless.Client({
    apiUrl: API_URL,
    apiKey: API_KEY,
  });

  const signupData = {
    userId: self.crypto.randomUUID(),
    username: username,
    aliases: aliases,
  };

  try {
    const response = await fetch("/passwordless/api/register", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(signupData),
    });

    if (!response.ok) {
      const errorData = await response.json();
      throw new Error(errorData.error || "Registration failed");
    }

    const backendResponse = await response.json();
    console.log("Registration successful:", backendResponse);

    const { token, error } = await p.register(backendResponse.token);
    if (token) {
      Toast.success(
        "Successfully registered your passkey!",
        "Registration Complete",
      );
      // Optional: redirect after successful registration
      setTimeout(() => {
        window.location.href = "/";
      }, 2000);
    } else {
      Toast.error(error || "Failed to register passkey", "Registration Error");
    }
  } catch (e) {
    console.error("Registration error:", e);
    Toast.error(
      e.message || "An unexpected error occurred",
      "Registration Error",
    );
  }
}

document
  .getElementById("passwordless-register")
  .addEventListener("click", handleRegisterClick);
