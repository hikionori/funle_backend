import { AbsoluteCenter, Button, Input, Text } from "@chakra-ui/react";
import Head from "next/head";
import React, { useEffect } from "react";
import { Content, Info, createInfo } from "../../utils/admin-sdk/info/index";
import LevelNodeList, {
    LevelNodeListProps,
} from "../../components/levelNodeList";
import AddOptionButton from "../../components/addOptionButton";
import LevelNode, { LevelNodeProps } from "../../components/levelNode";
import Node, { NodeProps } from "../../components/node";

import useTutorialStore, { TutorialState } from "../../utils/states/tutorial";

export default function CreateNewTutorial() {
    const {title, setTitle} = useTutorialStore((state: any) => ({title: state.title, setTitle: state.setTitle}));
    const {theme, setTheme} = useTutorialStore((state: any) => ({theme: state.theme, setTheme: state.setTheme}));
    const {contentLevels, setContentLevels} = useTutorialStore((state: any) => ({contentLevels: state.contentLevels, setContentLevels: state.setContentLevels}));

    const {addLevel} = useTutorialStore((state: any) => ({addLevel: state.addLevel}));

    useEffect(() => {
        setTitle("Test");
        setTheme("Test");
        setContentLevels([
            [
                1, // index of level
                [
                    {
                        index: 0,
                        content_type: "text",
                        data: "Test",
                        onDelete: () => {},
                    },
                    {
                        index: 1,
                        content_type: "image", // url to image
                        data: "https://bafybeif6lmz2jshbomqyjkvm2qrba7qsd4ywpawwbub6lyj2r665peqco4.ipfs.w3s.link/%D0%B7%D0%BD%D0%B0%D1%87%D0%BE%D0%BA.png",
                        onDelete() {},
                    },
                ],
            ],
            [
                0, // index of level
                [
                    {
                        index: 0,
                        content_type: "text",
                        data: "Test",
                        onDelete: () => {},
                    },
                ],
            ],
            [
                2, // index of level
                [
                    {
                        index: 0,
                        content_type: "text",
                        data: "Test",
                        onDelete: () => {},
                    },
                ],
            ],
        ]);
    }, []);

    // useEffect(() => {
    //     console.log(contentLevels);
    // }, [contentLevels]);

    return (
        <>
            <Head>
                <title>Create new tutorial</title>
            </Head>
            <main>
                <AbsoluteCenter>
                    <Input
                        value={title}
                        onChange={(e) => setTitle(e.target.value)}
                        border={"1px solid black"}
                        placeholder={"Title"}
                        focusBorderColor="orange.500"
                        _hover={{
                            borderColor: "orange.400",
                            bgColor: "orange.50",
                            _placeholder: { color: "blackAlpha.900" },
                        }}
                    />
                    <Input
                        marginTop={"10px"}
                        marginBottom={"10px"}
                        value={theme}
                        onChange={(e) => setTheme(e.target.value)}
                        border={"1px solid black"}
                        placeholder={"Theme"}
                        focusBorderColor="orange.500"
                        _hover={{
                            borderColor: "orange.400",
                            bgColor: "orange.50",
                            _placeholder: { color: "blackAlpha.900" },
                        }}
                    />

                    <LevelNodeList
                        nodeLevels={contentLevels}
                    />

                    <AddOptionButton
                        onClick={() => {
                            addLevel();
                        }}
                    />
                </AbsoluteCenter>
            </main>
        </>
    );
}
