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
    const [contentLevels, setContentLevels] = React.useState();

    useEffect(() => {
        setTitle("Test");
        setTheme("Test");
        setContentLevels([
            [
                0,
                [
                    {
                        index: 0,
                        content_type: "text",
                        data: "Test",
                        onEdit: () => {},
                        onDelete: () => {},
                    },
                ],
            ],
            [
                1,
                [
                    {
                        index: 0,
                        content_type: "text",
                        data: "Test",
                        onEdit: () => {},
                        onDelete: () => {},
                    },
                ],
            ],
        ] as any);
    }, []);

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

                    {/* List of levels */}
                    <LevelNode
                        index={0}
                        nodes={[
                            {
                                index: 0,
                                content_type: "text",
                                data: "Test",
                                onDelete(index) {
                                    console.log(
                                        "Delete node at index: " + index
                                    );
                                },
                                onEdit(index) {
                                    console.log("Edit node at index: " + index);
                                },
                            },
                            {
                                index: 1,
                                content_type: "text",
                                data: "Test",
                                onDelete(index) {
                                    console.log(
                                        "Delete node at index: " + index
                                    );
                                },
                                onEdit(index) {
                                    console.log("Edit node at index: " + index);
                                },
                            },
                            {
                                index: 2,
                                content_type: "text",
                                data: "Test",
                                onDelete(index) {
                                    console.log(
                                        "Delete node at index: " + index
                                    );
                                },
                                onEdit(index) {
                                    console.log("Edit node at index: " + index);
                                },
                            },
                        ]}
                    />

                    <AddOptionButton
                        onClick={() => {
                            // add one new empty level
                            let node = {
                                content_type: "text",
                                data: "",
                            } as NodeProps;
                            let level = [node] as unknown as LevelNodeProps;
                            setContentLevels((prev: any) => {
                                if (prev) {
                                    return [...prev, level];
                                } else {
                                    return [level];
                                }
                            });
                        }}
                    />
                </AbsoluteCenter>
            </main>
        </>
    );
}
