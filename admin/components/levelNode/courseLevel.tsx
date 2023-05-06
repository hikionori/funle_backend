import { Box, Flex, useEditable } from "@chakra-ui/react";
import { useEffect, useState } from "react";
import CourseNode from "../node/courseNode";

export interface CourceLevelProps {
    index: number;
    nodes: any;
}

interface CourseLevelFunctions {
    deleteHandler: Function;
}

export default function CourceLevel(
    props: CourceLevelProps & CourseLevelFunctions
) {
    const [nodes, setNodes] = useState([]);

    const deleteHandler = props.deleteHandler;

    const levelIndex = props.index;

    useEffect(() => {
        setNodes(props.nodes);
    }, []);

    return (
        <Box w={"100%"}>
            <Flex direction={"row"} w="100%">
                {props.nodes &&
                    props.nodes.map((node: any, index: number) => {
                        return (
                            <CourseNode
                                index={index}
                                id={node.id}
                                title={node.title}
                                ids={node.ids}
                                mini_image={node.mini_image}
                                type_={node.type_}
                                key={index}
                                n_of_tests={node.n_of_tests}
                                onDelete={() => {
                                    deleteHandler(levelIndex, node.index);
                                }}
                            />
                        );
                    })}
            </Flex>
        </Box>
    );
}
