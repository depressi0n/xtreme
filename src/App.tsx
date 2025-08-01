import React from 'react';
import {Input} from "@/components/ui/input";
import {cn} from "@/lib/utils"
import "./App.css";

function App() {
    // const [greetMsg, setGreetMsg] = useState("");
    // const [name, setName] = useState("");
    //
    // async function greet() {
    //     // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    //     setGreetMsg(await invoke("greet", {name}));
    // }

    return (
        <React.Fragment >
            <div
                data-tauri-drag-region
                className={cn(
                    "flex justify-between items-center",
                    "p-2 pl-0",
                )}
            >
                <Input
                    autoFocus={true}
                    className={cn(
                        "w-1/2 h-10 p-0 pl-2 ml-2",
                        "text-foreground",
                        "outline-none shadow-none border-none",
                    )}
                    onChange={async (_e) => {
                    }}
                    placeholder="Search for apps and commands..."
                />
            </div>
        </React.Fragment>
    );
}

export default App;
