import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [success, setMessage] = useState("");
  const [text, setText] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setMessage(await invoke("set_text", { text }));
  }

  return (
    <main className="container">
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setText(e.currentTarget.value)}
          placeholder="Enter a text to display"
        />
        <button type="submit">Push</button>
      </form>
      <p>{success}</p>
    </main>
  );
}

export default App;
