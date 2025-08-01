import React, {useEffect, useState} from 'react';
import {invoke} from "@tauri-apps/api/core";

import {cn} from "@/lib/utils"
import "./App.css";
import {Input} from "@/components/ui/input";
import {Separator} from "@/components/ui/separator";
import {Button} from "@/components/ui/button";
import {Tooltip, TooltipTrigger, TooltipContent} from "@/components/ui/tooltip";

function App() {
    const [plugins, setPlugins] = useState<PluginInfo[]>([]);

    const [query, setQuery] = useState("");
    const [results, setResults] = useState<{
        id: string;
        title: string;
        description: string;
    }[]>([]);


    useEffect(() => {
        // 启动时加载插件
        const loadAllPlugins = async () => {
            try {
                const loadedPlugins: PluginInfo[] = await invoke('load_plugins');
                setPlugins(loadedPlugins);
                console.log("Plugins loaded:", loadedPlugins);
            } catch (error) {
                console.error("Failed to load plugins:", error);
            }
        };
        loadAllPlugins();
    }, []);

    useEffect(() => {
        if (query.startsWith('>')) {
            const searchTerm = query.substring(1).toLowerCase();
            // 过滤插件命令
            const filteredPlugins = plugins.filter(plugin =>
                plugin.name.toLowerCase().includes(searchTerm) ||
                plugin.command.toLowerCase().includes(searchTerm)
            );
            setResults(filteredPlugins.map(p => ({
                id: p.command,
                title: p.name,
                description: p.description
            })));
        } else if (query.length > 0) {
            // 模拟普通搜索结果
            setResults([{ id: 'search', title: `Search for "${query}"`, description: 'Execute a web search.' }]);
        } else {
            // 当输入为空时，显示所有可用插件作为“建议”
            setResults(plugins.map(p => ({
                id: p.command,
                title: p.name,
                description: p.description
            })));
        }
    }, [query, plugins]);

    const runPluginCommand = async (commandName: string) => {
        try {
            const response = await invoke<string>('run_plugin_command', { commandName });
            console.log('Plugin command executed:', response);
            return response;
        } catch (error) {
            console.error('Failed to run plugin command:', error);
        }
    };

    const executeBuiltinCommand = async (command: string) => {
        // 调用 Tauri 后端，传入完整的命令字符串
        try {
            const response = await invoke('open_url', {command: command});
            console.log('Command executed successfully:', response);
        } catch (error) {
            console.error('Failed to execute command:', error);
        }
    };

    // 当用户按下 Enter 键时执行命令
    const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key === 'Enter') {
            const commandToRun = query.startsWith('>') ? query.substring(1) : query;
            // 尝试匹配插件命令
            console.log("commandToRun: ", commandToRun);
            const matchedPlugin = plugins.find(p => p.command === commandToRun);
            if (matchedPlugin) {
                runPluginCommand(matchedPlugin.command).then((r)=>{
                    console.log(r)
                });
            } else {
                // 执行普通的搜索或命令
                executeBuiltinCommand(query);
            }
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
