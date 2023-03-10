import {
    Box,
    Stack,
    Text,
    Button,
    ButtonGroup,
    Center,
} from "@chakra-ui/react";
import { TestType } from "../../utils/admin-sdk/tests/index";
import { FiTrash, FiEdit } from "react-icons/fi";
import React from "react";

interface TestCardProps {
    id: string;
    text: string;
    type: TestType;
    onClick: (id: string) => void;
    onDelete?: (id: string) => void;
}

export default function TestCard(props: TestCardProps) {
    const { id, text, type, onClick } = props;
    const onDelete = props.onDelete;

    return (
        <>
            <Box height={"80px"}>
                <Stack
                    height={"100%"}
                    borderColor={"black"}
                    borderRadius={"xl"}
                    backgroundColor={"blue.100"}
                    justifyContent={"space-around"}
                    direction={"row"}
                    textAlign={"center"}
                >
                    <Center>
                        <Text>id: {id}</Text>
                    </Center>
                    <Center>
                        <Text>task: {text}</Text>
                    </Center>
                    <Center>
                        <Text>type : {type}</Text>
                    </Center>

                    <ButtonGroup
                        flexDir={"row"}
                        alignSelf={"center"}
                        justifyContent={"space-around"}
                        width={"40"}
                    >
                        <Button onClick={() => onClick(id)}>
                            <FiEdit />
                        </Button>

                        {onDelete && (
                            <Button onClick={() => onDelete(id)}>
                                <FiTrash />
                            </Button>
                        )}
                    </ButtonGroup>
                </Stack>
            </Box>
        </>
    );
}
