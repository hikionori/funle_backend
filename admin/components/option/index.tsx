import {
    Box,
    Button,
    Checkbox,
    Divider,
    HStack,
    Input,
    Spacer,
    Text,
} from "@chakra-ui/react";
import { useEffect, useState } from "react";
import { FaTrash } from "react-icons/fa";

interface OptionCardProps {
    data: any;
    type: string;
    index: number;
    onEdit: (data: any) => void;
    onDelete?: (index: number) => void;
}

export default function OptionCard(props: OptionCardProps) {
    const [text, setText] = useState<string>(props.data.text);
    const [isCorrect, setIsCorrect] = useState<boolean>(props.data?.isCorrect);

    const typeIs = props.type;
    const onDelete = props.onDelete;
    const onEdit = props.onEdit;
    const index = props.index;

    useEffect(() => {
        if (typeIs==="choice") {
            onEdit({ text, isCorrect });
        } else {
            onEdit({ text });
        }
    }, [text, isCorrect]);

    useEffect(() => {
        setText(props.data.text);
        setIsCorrect(props.data?.isCorrect);
    }, [props.data]);

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
                        color="black"
                        onClick={() => {
                            if (onDelete) onDelete(index);
                        }}
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
                        onChange={(e) => setText(e.target.value)}
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
                    color="black"
                    bgColor="red.200"
                    onClick={() => {
                        if (onDelete) onDelete(props.index);
                    }}
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
                    onChange={(e) => setText(e.target.value)}
                ></Input>
                <Box width={"25px"} />
            </HStack>
        </HStack>
    );
}
