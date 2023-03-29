import {
    Box,
    Stack,
    Text,
    Button,
    ButtonGroup,
    Center,
    Flex,
    Spacer,
    HStack,
} from "@chakra-ui/react";
import { TestType } from "../../utils/admin-sdk/tests/index";
import { FiTrash, FiEdit } from "react-icons/fi";
import React from "react";

export interface TutorialCardProps {
    id: string;
    title: string;
    onClick: Function;
    onDelete?: Function;
}

export default function TutorialCard(props: TutorialCardProps) {
    const { id, title, onClick } = props;
    const onDelete = props?.onDelete;

    return (
        <Box height={"80px"} paddingTop="3">
            <Flex
                height={"100%"}
                borderWidth={"thin"}
                borderColor={"blackAlpha.400"}
                borderRadius={"xl"}
                backgroundColor={"whiteAlpha.400"}
                // justifyContent={"space-around"}
                direction={"row"}
                textAlign={"center"}
            >
                <HStack spacing={"20"}>
                    <Center paddingLeft={"30px"}>
                        <Text>id: {id}</Text>
                    </Center>
                    <Spacer />
                    <Center paddingRight={"20"}>
                        <Text>Title: {title}</Text>
                    </Center>
                </HStack>
                <Spacer />
                <ButtonGroup
                    flexDir={"row"}
                    alignSelf={"center"}
                    justifyContent={"space-around"}
                    paddingRight={"20"}
                >
                    <Button
                        bgColor={"blackAlpha.200"}
                        color="black"
                        onClick={() => onClick(id)}
                        _hover={{ color: "white", bgColor: "orange.400" }}
                    >
                        <FiEdit />
                    </Button>

                    {onDelete && (
                        <Button
                            bgColor={"blackAlpha.200"}
                            color="black"
                            _hover={{ color: "white", bgColor: "red.600" }}
                            onClick={() => onDelete(id)}
                        >
                            <FiTrash />
                        </Button>
                    )}
                </ButtonGroup>
            </Flex>
        </Box>
    );
}