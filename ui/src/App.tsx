import {useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import "./App.css";
import Select, {SingleValue} from 'react-select'

const mode_options = [
    {value: 'left', label: 'Left'},
    {value: 'right', label: 'Right'},
    {value: 'up', label: 'Up'},
    {value: 'down', label: 'Down'},
    {value: 'center', label: 'Center'},
    {value: 'fast', label: 'Fast'},
    {value: 'drop', label: 'Drop'},
    {value: 'curtain', label: 'Curtain'},
    {value: 'laser', label: 'Laser'},
]

interface ModeProps {
    onChange: (e: SingleValue<{ value: string; label: string }>) => void
}

let StyleSelect = ({onChange}: ModeProps) => (
    <Select options={mode_options} defaultValue={{value: 'left', label: 'Left'}}
            onChange={onChange}/>
)

function App() {
    const [success, setMessage] = useState("");
    const [text, setText] = useState("");
    const [speed, setSpeed] = useState(4);
    const [mode, setMode] = useState("");

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        setMessage(await invoke("set_text", {text, speed, mode
    }))
        ;
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
                    id="text"
                    onChange={(e) => setText(e.currentTarget.value)}
                    placeholder="Enter a text to display"
                />
                <p>Speed:</p>
                <button type="submit">Push</button>
                <br/>
                <input id="speed" type="number" min={0} max={7} defaultValue={4}
                       onChange={(e) => setSpeed(parseInt(e.currentTarget.value))}/>
            </form>
            <StyleSelect onChange={(e: SingleValue<{ value: string; label: string }>) => {
                if (e) setMode(e.value)
            }}/>

            <p>{success}</p>
        </main>
    );
}

export default App;
