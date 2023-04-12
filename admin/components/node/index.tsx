import {
    AbsoluteCenter,
    Box,
    Button,
    Center,
    Flex,
    Text,
} from "@chakra-ui/react";
import { useState } from "react";

export interface NodeProps {
    index: number;
    content_type: string;
    data: string;

    onEdit: (index: number) => void;
    onDelete: (index: number) => void;
}

export default function Node(props: NodeProps) {
    const { content_type, data, index} = props;
    const { onEdit, onDelete } = props;

    const [hovered, setHovered] = useState(false);

    const deleteHandler = () => {
        onDelete(index);
    };

    const editHandler = () => {
        onEdit(index);
    }

    const onMouseHover = () => {
        setHovered(true);
    };

    const onMouseLeave = () => {
        setHovered(false);
    };

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
            {/* if hovered is true return box else another box*/}
            {!hovered ? (
                <Box w={"100%"} h={"100px"}>
                    <Text>{content_type}</Text>
                    <Text>{data}</Text>
                </Box>
            ) : (
                <Box w={"100%"} h={"100px"}>
                    <Center>
                        <Flex direction={"row"}>
                            <Button onClick={editHandler} size={"sm"}>
                                Edit
                            </Button>
                            <Button onClick={deleteHandler} size={"sm"}>
                                Delete
                            </Button>
                        </Flex>
                    </Center>
                </Box>
            )}
        </Box>
    );
}
