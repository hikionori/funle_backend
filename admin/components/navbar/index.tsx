// NextJS drawer navigation component using TailwindCSS

import React from "react";
import Link from "next/link";
import { useRouter } from "next/router";

export default function Navbar() {
    const router = useRouter();

    return (
        // Drawer navigation. Position: fixed-left. Width: 64px
        <div className="fixed left-0 top-0 h-screen w-80 bg-gray-800 text-white">
            <div className="flex flex-col h-full">
                <div className="flex font-mono font-bold text-4xl flex-col justify-center items-center mt-20">
                    <p className="self-center">FunLe</p>
                </div>
                {/* 3 Drawer items */}
                <div className="flex flex-col h-40 mt-20">
                    <div className="flex flex-row h-full font-mono text-center justify-evenly">
                        <span>Icon</span>
                        <span>Test</span>
                    </div>
                    <div className="flex flex-row h-full font-mono text-center justify-evenly">
                        <span>Icon</span>
                        <span>Infos</span>
                    </div>
                    <div className="flex flex-row h-full font-mono text-center justify-evenly">
                        <span>Icon</span>
                        <span>Cources</span>
                    </div>
                </div>
            </div>
        </div>
    );
}
