import React, {useEffect, useState} from 'react';
import {invoke} from "@tauri-apps/api/core";

import {cn} from "@/lib/utils"
import "./App.css";
import {Input} from "@/components/ui/input";
import {Separator} from "@/components/ui/separator";
import {Button} from "@/components/ui/button";
import {Tooltip, TooltipTrigger, TooltipContent} from "@/components/ui/tooltip";

// 模拟命令数据
const availableCommands = [
    {id: '1', title: 'Open Google', command: '>google', description: 'Opens Google.com in your default browser.'},
    {id: '2', title: 'Search on Wikipedia', command: '>wiki', description: 'Search on Wikipedia for a given term.'},
];

function App() {
    const [query, setQuery] = useState("");
    const [results, setResults] = useState<any[]>([]);
    const [isCommandMode, setIsCommandMode] = useState(false);

    useEffect(() => {
        // 检查是否进入命令模式
        setIsCommandMode(query.startsWith('>'));

        if (isCommandMode) {
            // 过滤命令
            const searchTerm = query.substring(1).toLowerCase();
            const filteredCommands = availableCommands.filter(cmd =>
                cmd.title.toLowerCase().includes(searchTerm) || cmd.command.toLowerCase().includes(searchTerm)
            );
            setResults(filteredCommands);
        } else if (query.length > 0) {
            // 模拟普通搜索结果
            const fakeResults = [
                {id: '3', title: `Search for "${query}"`, description: 'Execute a web search.'},
            ];
            setResults(fakeResults);
        } else {
            setResults([]);
        }
    }, [query, isCommandMode]);

    const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key === 'Enter') {
            if (isCommandMode) {
                openURL(query);
            } else {
                // 执行普通搜索
                console.log(`Executing web search for: ${query}`);
            }
        }
    };

    const openURL = async (command: string) => {
        // 调用 Tauri 后端，传入完整的命令字符串
        try {
            const response = await invoke('open_url', {command: command});
            console.log('Command executed successfully:', response);
        } catch (error) {
            console.error('Failed to execute command:', error);
        }
    };

    return (
        <React.Fragment>
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
                    onChange={(e) => {
                        setQuery(e.target.value);
                    }}
                    onKeyDown={handleKeyDown}
                    placeholder="Search for apps and commands..."
                />
            </div>
            <div className="flex justify-center">
                <Separator className="w-full bg-gray-500 dark:bg-gray-50 h-[1px]"/>
            </div>
            <div className="w-1/3 flex flex-col">
                {results.length > 0 &&
                    results.map((cmd) => (
                        <Tooltip key={cmd.id} >
                            <TooltipTrigger asChild>
                                <Button
                                    variant="outline"
                                    className={cn("shadow-none")}
                                >
                                    {cmd.title}
                                </Button>
                            </TooltipTrigger>
                            <TooltipContent>
                                <p>{cmd.description}</p>
                            </TooltipContent>
                        </Tooltip>
                    ))}
            </div>
        </React.Fragment>
    );
}

export default App;
