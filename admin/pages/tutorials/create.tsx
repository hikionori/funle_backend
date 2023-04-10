import { AbsoluteCenter, Button, Input, Text } from "@chakra-ui/react";
import Head from "next/head";
import React, { useEffect } from "react";
import { Content, Info, createInfo } from "../../utils/admin-sdk/info/index";
import LevelNode from "../../components/levelNode";
import LevelNodeTree from "../../components/levelNodeTree";
import AddOptionButton from "../../components/addOptionButton";

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
    */

    /*
        Display content levels:
                    input title
                    input theme
                Block for first level of content
                if (contentLevels.length == 2) {
                    split contentLevels into two row
                }
                if (contentLevels.length == 3) {
                    split contentLevels into three row
                }
                if (contentLevels.length == 4) {
                    split contentLevels into four row
                }
    */

    const [title, setTitle] = React.useState("");
    const [theme, setTheme] = React.useState("");
    const [contentLevels, setContentLevels] = React.useState<Content[][]>([]);

    useEffect(() => {
        setTitle("Test");
        setTheme("Test");
        setContentLevels([
            [
                {
                    content_type: "text",
                    data: "Test", // to bytes
                },
            ],
            [
                {
                    content_type: "text",
                    data: "Test",
                },
                {
                    content_type: "text",
                    data: "Test",
                },
            ],
        ]);
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
                    
                   <AddOptionButton
                        onClick={() => {
                            setContentLevels((prev) => {
                                const temp = [...prev];
                                temp.push([]);
                                return temp;
                            });
                        }}
                    />
                </AbsoluteCenter>
            </main>
        </>
    );
}
