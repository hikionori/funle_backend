import { Box, Center, Input, Select, Stack, Text } from "@chakra-ui/react";
import { useState } from "react";

export interface CourseNodeData {
    id: string;
    ids: string[];
    title: string;
    mini_image: string;
    type_: string;
    // if type_ is test
    n_of_tests?: number;
}

export default function CourseNode(props: CourseNodeData) {
    const [id, setId] = useState(props.id);
    const [ids, setIds] = useState(props.ids);
    const [title, setTitle] = useState(props.title);
    const [mini_image, setMini_image] = useState(props.mini_image);
    const [type_, setType_] = useState(props.type_);
    const [n_of_tests, setN_of_tests] = useState(props.n_of_tests);

    return (
        <Center>
            <Box
                backgroundColor={"white"}
                borderRadius={"10px"}
                padding={"10px"}
                w={"100%"}
            >
                <Stack
                spacing={5}
                w={"100%"}
                >
                    <Select
                        value={type_}
                        onChange={(e) => {
                            setType_(e.target.value);
                        }}
                    >
                        <option value="info">Info</option>
                        <option value="test">Test</option>
                    </Select>
                    <Input
                        value={title}
                        onChange={(e) => {
                            setTitle(e.target.value);
                        }}
                    />
                    <Input
                        value={mini_image}
                        onChange={(e) => {
                            setMini_image(e.target.value);
                        }}
                    />
                    {type_ === "test" ? (
                        <Input
                            value={n_of_tests}
                            placeholder="Number of tests"
                            onChange={(e) => {
                                setN_of_tests(parseInt(e.target.value));
                            }}
                        />
                    ) : (
                        <></>
                    )}
                </Stack>
            </Box>
        </Center>
    );
}
