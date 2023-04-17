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

export default function CreateNewTutorial() {
    /* 
        API Request body:
        {
            title: string,
            theme: string,
            content_levels: [
                content_type: string, // text, image
                data: string // text, image base64
            ][]
        }

        Algorithm for sending request:
        1. Remove all empty nodes
        2. Remove all empty levels
        3. Remove index from each node
        4. remove onEdit and onDelete from each node
    */

    const [title, setTitle] = React.useState("");
    const [theme, setTheme] = React.useState("");
    const [contentLevels, setContentLevels] = React.useState<LevelNodeProps[]>(
        []
    );

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
                    }
                ]
            ]
        ] as unknown as LevelNodeProps[]);
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

                    <LevelNodeList nodeLevels={contentLevels} />

                    <AddOptionButton
                        onClick={() => {
                            // get last level index
                            let lastLevelIndex: number =
                                contentLevels.length - 1;
                            // get last level
                            let lastLevel: any = contentLevels[lastLevelIndex];
                            lastLevelIndex = lastLevel[0];

                            // add one new empty level
                            let node = {
                                content_type: "text",
                                data: "",
                            } as NodeProps;
                            let level = [
                                lastLevelIndex + 1,
                                node,
                            ] as unknown as LevelNodeProps;
                            // push level to contentLevels array
                            setContentLevels([...contentLevels, level]);
                        }}
                    />
                </AbsoluteCenter>
            </main>
        </>
    );
}
