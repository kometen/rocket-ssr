async function handleSigninClick(e) {
  e.preventDefault();

  Toast.info("Starting sign in process...", "Authentication");

  const p = new Passwordless.Client({
    apiUrl: API_URL,
    apiKey: API_KEY,
  });

  try {
    const { token, error } = await p.signinWithAutofill();
    if (error) {
      Toast.error(error, "Sign In Failed");
      return;
    }

    console.log("Received token", token);
    Toast.info("Verifying with server...", "Authentication");

    const response = await fetch("/passwordless/api/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        token: token,
      }),
    });

    const user = await response.json();

    if (response.ok) {
      Toast.success("Successfully signed in!", "Welcome");
      // Reload page after a short delay to allow the toast to be seen
      setTimeout(() => {
        window.location.reload();
      }, 1000);
    } else {
      Toast.error(user.error || "Sign in failed", "Authentication Error");
    }
  } catch (e) {
    console.error("Sign in error:", e);
    Toast.error(
      e.message || "An unexpected error occurred",
      "Authentication Error",
    );
  }
}

document
  .getElementById("passwordless-signin")
  .addEventListener("click", handleSigninClick);
