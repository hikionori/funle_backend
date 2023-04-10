import { Box, Text } from "@chakra-ui/react";
import { Content } from "../../utils/admin-sdk";
import Image from "next/image";


export interface LevelNodeProps {
    content: Content;
}

export default function LevelNode(props: LevelNodeProps) {
    const { content } = props;
    /**
     * Display content levels:
     * if content type is text
     *     display text
     * if content type is image
     *    display image
     */
    return (
        <Box> {/* container */}
            <Box> {/* display data box */}

            </Box>
            <Box> {/* on hover action box */}

            </Box>
        </Box>
    );

}