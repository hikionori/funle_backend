import { Box, Button, Center } from "@chakra-ui/react";
import { FaPlus } from "react-icons/fa";

export interface AddOptionButtonProps {
    _type?: "choice" | "action";
    onClick: () => void;
}

export default function AddOptionButton({
    _type,
    onClick,
}: AddOptionButtonProps) {
    if (_type) {
        return (
            <Center>
                <Box w="100%" marginTop={"10px"}>
                    <Button
                        onClick={onClick}
                        colorScheme="orange"
                        variant="solid"
                        w="100%"
                    >
                        <FaPlus />
                    </Button>
                </Box>
            </Center>
        );
    } else {
        return (
            <Center>
                <Box w="100%" marginTop={"10px"}>
                    <Button
                        onClick={onClick}
                        colorScheme="orange"
                        variant="solid"
                        w="100%"
                    >
                        <FaPlus />
                    </Button>
                </Box>
            </Center>
        );
    }
}
