import {
    AbsoluteCenter,
    Box,
    Button,
    Center,
    Flex,
    Image,
    Input,
    Select,
    Switch,
    Text,
    Textarea,
} from "@chakra-ui/react";
import { useEffect, useState } from "react";

import useTutorialStore from "../../utils/states/tutorial";

export interface NodeProps {
    index: number;
    content_type: string;
    data: string;
}

export interface NodeFC {
    levelIndex: number;
    editHandler: any; // Function;
    deleteHandler: any; // Function;
}

export default function Node(props: NodeProps & NodeFC) {
    const { content_type, data, index } = props;

    const [hovered, setHovered] = useState(false);
    const [content, setContent] = useState(data);
    const [contentType, setContentType] = useState(content_type);
    const [edit, setEdit] = useState(false);

    const {editNode} = useTutorialStore((state: any) => ({editNode: state.editNode}));

    const onMouseHover = () => {
        setHovered(true);
    };

    const onMouseLeave = () => {
        setHovered(false);
    };

    if (!edit) {
        if (content_type === "text") {
            return (
                <Box
                    background={"#F5F5F5"}
                    borderRadius={"10px"}
                    padding={"10px"}
                    border={"1px solid #E0E0E0"}
                    onMouseEnter={onMouseHover}
                    onMouseLeave={onMouseLeave}
                    w={"full"}
                    margin={"5px"}
                >
                    {/* if edit is false display */}
                    {!hovered ? (
                        <Box w={"100px"} h={"100px"}>
                            <Text>{contentType}</Text>
                            <Text>{content}</Text>
                        </Box>
                    ) : (
                        <Box w={"100%"} h={"100px"}>
                            <Center>
                                <Flex direction={"row"}>
                                    <Button
                                        onClick={() => {
                                            setEdit(true);
                                        }}
                                        size={"sm"}
                                    >
                                        Edit
                                    </Button>
                                    <Button
                                        onClick={props.deleteHandler}
                                        size={"sm"}
                                    >
                                        Delete
                                    </Button>
                                </Flex>
                            </Center>
                        </Box>
                    )}
                </Box>
            );
        }
        if (content_type === "image") {
            return (
                <Box
                    background={"#F5F5F5"}
                    borderRadius={"10px"}
                    padding={"10px"}
                    border={"1px solid #E0E0E0"}
                    onMouseEnter={onMouseHover}
                    onMouseLeave={onMouseLeave}
                    w={"full"}
                    margin={"5px"}
                >
                    {hovered ? (
                        <Box w={"100%"} h={"100px"}>
                            <Center>
                                <Flex direction={"row"}>
                                    <Button
                                        onClick={() => {
                                            setEdit(true);
                                        }}
                                        size={"sm"}
                                    >
                                        Edit
                                    </Button>
                                    <Button
                                        onClick={props.deleteHandler}
                                        size={"sm"}
                                    >
                                        Delete
                                    </Button>
                                </Flex>
                            </Center>
                        </Box>
                    ) : (
                        <Box w={"100%"} h={"100px"}>
                            <Text>{contentType}</Text>
                            <Image
                                src={content}
                                alt={"image"}
                                width={"60px"}
                                height={"60px"}
                                resize={"both"}
                            />
                        </Box>
                    )}
                </Box>
            );
        } else {
            return (
                <Box
                    background={"#F5F5F5"}
                    borderRadius={"10px"}
                    padding={"10px"}
                    border={"1px solid #E0E0E0"}
                    onMouseEnter={onMouseHover}
                    onMouseLeave={onMouseLeave}
                    w={"full"}
                    margin={"5px"}
                >
                    <Text>Unsuported content type: {content_type}</Text>
                </Box>
            );
        }
    } else {
        return (
            <Box
                background={"#F5F5F5"}
                borderRadius={"10px"}
                padding={"10px"}
                border={"1px solid #E0E0E0"}
                onMouseEnter={onMouseHover}
                onMouseLeave={onMouseLeave}
                w={"full"}
                margin={"5px"}
            >
                <Box w={"100%"} h={"fit-content"}>
                    {/* drop down choice for choice between text and image */}
                    <Select
                        placeholder="Select type of data"
                        mb={"10px"}
                        onChange={(e) => {
                            setContentType(e.target.value.toLowerCase());
                        }}
                        value={contentType}
                    >
                        <option value="text">Text</option>
                        <option value="image">Image</option>
                    </Select>

                    <Textarea
                        value={content}
                        onChange={(e) => setContent(e.target.value)}
                        border={"1px solid black"}
                        placeholder={"Title"}
                        focusBorderColor="orange.500"
                        _hover={{
                            borderColor: "orange.400",
                            bgColor: "orange.50",
                            _placeholder: { color: "blackAlpha.900" },
                        }}
                        onKeyDown={(e) => {
                            if (e.key === "Enter") {
                                editNode(props.levelIndex, index, contentType, content)
                                setEdit(false);
                                console.log(content);
                            }
                            // on shift + enter add new line
                            if (e.shiftKey && e.key === "Enter" && edit) {
                                setContent(content + "\n");
                                setEdit(true);
                            }
                        }}
                    />
                </Box>
            </Box>
        );
    }
}
