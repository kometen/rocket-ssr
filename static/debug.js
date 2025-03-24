if (!API_KEY || API_KEY[0] === "<" || API_KEY === "YOUR_API_KEY") {
  Toast.warning(
    "Missing API Key. Please configure API_KEY and API_KEY_SECRET before running the application.",
    "Configuration Warning",
  );
}
