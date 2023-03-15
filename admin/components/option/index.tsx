import {
    Box,
    Button,
    Checkbox,
    Divider,
    HStack,
    Input,
    Spacer,
} from "@chakra-ui/react";
import { useState } from "react";
import { FaTrash } from "react-icons/fa";

interface OptionCardProps {
    type: string;
    onDelete: () => void;
}

export default function OptionCard(props: OptionCardProps) {
    const [text, setText] = useState<string>("");
    const [isEditing, setIsEditing] = useState<boolean>(false);
    const [isCorrect, setIsCorrect] = useState<boolean>(false);

    const typeIs = props.type;
    const onDelete = props.onDelete;

    if (typeIs === "choice") {
        return (
            <HStack marginLeft={"30px"} width="full">
                <Divider
                    orientation="vertical"
                    borderColor="blackAlpha.400"
                    height={"50px"}
                    width={"3px"}
                />
                <Spacer />
                <HStack marginBottom={"10px"}>
                    <Button
                        onClick={onDelete}
                        color="black"
                        bgColor="red.200"
                        _hover={{
                            bgColor: "red.300",
                            border: "2px solid red.500",
                            borderColor: "red.500",
                            color: "white",
                            transition: "all 0.2s",
                        }}
                    >
                        <FaTrash />
                    </Button>
                    <Input
                        value={text}
                        focusBorderColor={"orange.400"}
                        border={"1px solid black"}
                        placeholder="Choice Variant"
                        _hover={{
                            borderColor: "orange.400",
                        }}
                    ></Input>
                    <Checkbox
                        isChecked={isCorrect}
                        onChange={() => setIsCorrect(!isCorrect)}
                        border="1px solid black"
                        borderRadius={"5"}
                        size={"lg"}
                        colorScheme="orange"
                    />
                    <Box width={"25px"} />
                </HStack>
            </HStack>
        );
    }
    return (
        <HStack marginLeft={"30px"}>
            <Divider
                orientation="vertical"
                borderColor="blackAlpha.400"
                height={"50px"}
                width={"3px"}
            />
            <Spacer />
            <HStack marginBottom={"10px"}>
                <Button
                    onClick={onDelete}
                    color="black"
                    bgColor="red.200"
                    _hover={{
                        bgColor: "red.300",
                        border: "2px solid red.500",
                        borderColor: "red.500",
                        color: "white",
                        transition: "all 0.2s",
                    }}
                >
                    <FaTrash />
                </Button>
                <Input
                    value={text}
                    focusBorderColor={"orange.400"}
                    border={"1px solid black"}
                    placeholder="Action Variant"
                    _hover={{
                        borderColor: "orange.400",
                    }}
                ></Input>
                <Box width={"25px"} />
            </HStack>
        </HStack>
    );
}
