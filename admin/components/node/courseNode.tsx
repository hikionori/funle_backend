import {
    AbsoluteCenter,
    Box,
    Button,
    Center,
    Input,
    Modal,
    ModalBody,
    ModalContent,
    ModalFooter,
    ModalHeader,
    ModalOverlay,
    Select,
    Stack,
    Text,
} from "@chakra-ui/react";
import { useEffect, useState } from "react";
import {getAllInfos, getAllTests } from "../../utils/admin-sdk";

export interface CourseNodeData {
    index: number;
    id: string;
    ids: string[];
    title: string;
    mini_image: string;
    type_: string;
    // if type_ is test
    n_of_tests?: number;
}

interface CourseNodeFunctions {
    onDelete?: any; // Function
}

/**
 * Draft:
 *  - if type_ is info display 1 select for info id,
 *  - if type_ is test display n_of_tests selects for test ids
 */

export default function CourseNode(
    props: CourseNodeData & CourseNodeFunctions
) {
    // data
    const [id, setId] = useState(props.id);
    const [ids, setIds] = useState(props.ids);
    const [title, setTitle] = useState(props.title);
    const [mini_image, setMini_image] = useState(props.mini_image);
    const [type_, setType_] = useState(props.type_);
    const [n_of_tests, setN_of_tests] = useState(props.n_of_tests);

    const [ids_for_select, setIds_for_select] = useState<string[]>([]);

    // on hover interaction
    const [hover, setHover] = useState(false);

    const onMouseEnter = () => {
        setHover(true);
    };

    const onMouseLeave = () => {
        setHover(false);
    };

    // modal interaction
    const [isOpen, setIsOpen] = useState(false);
    const onClose = () => setIsOpen(false);
    const onOpen = () => setIsOpen(true);

    // delete
    const onDelete = props.onDelete;


    // if type_ is test get all tests and set ids_for_select to their ids
    useEffect(() => {
        if (type_ === "test") {
            prepareTestData();
        }
        // if type_ is info get all infos and set ids_for_select to their ids
        else if (type_ === "info") {
            prepareInfoData()
        }
    }, [type_]);

    const prepareInfoData = async () => {
        // set ids_for_select to all infos ids
        const infos = await getAllInfos();
        const ids_for_select: string[] = [];
        for (let i = 0; i < infos.length; i++) {
            ids_for_select.push(infos[i]._id["$oid"]);
        }
        setIds_for_select(ids_for_select);
    }

    const prepareTestData = async () => {
        const allTests = await getAllTests();
        const ids_for_select: string[] = [];
        const tests_ids = allTests.tests.map((test) => test._id["$oid"]);
        const test_a_ids = allTests.tests_with_actions.map((test) => test._id["$oid"]);

        // set ids_for_select to all tests ids
        for (let i = 0; i < tests_ids.length; i++) {
            ids_for_select.push(tests_ids[i]);
        }
        // set ids_for_select to all tests with actions ids
        for (let i = 0; i < test_a_ids.length; i++) {
            ids_for_select.push(test_a_ids[i]);
        }
        // shuffle ids_for_select
        for (let i = ids_for_select.length - 1; i > 0; i--) {
            const j = Math.floor(Math.random() * i);
            const temp = ids_for_select[i];
            ids_for_select[i] = ids_for_select[j];
            ids_for_select[j] = temp;
        }
        setIds_for_select(ids_for_select);
    }

    if (!hover) {
        return (
            <Center>
                <Box
                    backgroundColor={"white"}
                    borderRadius={"10px"}
                    padding={"10px"}
                    w={"100%"}
                    onMouseEnter={onMouseEnter}
                    h={"fit-content"}
                    margin={"5px"}
                >
                        <Input
                            value={id}
                            disabled
                            _disabled={{
                                backgroundColor: "gray.100",
                                textColor: "black",
                            }}
                        />
                        <Select
                            value={type_}
                            onChange={(e) => {
                                setType_(e.target.value);
                            }}
                            _disabled={{
                                backgroundColor: "gray.100",
                                textColor: "black",
                            }}
                            disabled
                        >
                            <option value="info">Info</option>
                            <option value="test">Test</option>
                        </Select>
                        <Input
                            value={title}
                            onChange={(e) => {
                                setTitle(e.target.value);
                            }}
                            disabled
                            _disabled={{
                                backgroundColor: "gray.100",
                                textColor: "black",
                            }}
                            placeholder="Title"
                        />
                        <Input
                            value={mini_image}
                            onChange={(e) => {
                                setMini_image(e.target.value);
                            }}
                            disabled
                            _disabled={{
                                backgroundColor: "gray.100",
                                textColor: "black",
                            }}
                            placeholder="Mini image url"
                        />
                        {type_ === "test" ? (
                            <Input
                                value={n_of_tests}
                                placeholder="Number of tests"
                                onChange={(e) => {
                                    setN_of_tests(parseInt(e.target.value));
                                }}
                                disabled
                                _disabled={{
                                    backgroundColor: "gray.100",
                                    textColor: "black",
                                }}
                            />
                        ) : (
                            <></>
                        )}
                </Box>
            </Center>
        );
    } else {
        return (
            <>
                <Box
                    backgroundColor={"white"}
                    borderRadius={"10px"}
                    padding={"10px"}
                    w={"100%"}
                    onMouseLeave={onMouseLeave}
                    h={type_ === "info" ? "160px" : "205px"}
                >
                    <Center>
                        <Stack
                            w={"fit-content"}
                            flexDirection={"row"}
                            justifyContent={"center"}
                        >
                            <Button onClick={onOpen}>Edit</Button>
                            <Button onClick={onDelete}>Delete</Button>
                        </Stack>
                    </Center>
                </Box>

                <Modal isOpen={isOpen} onClose={onClose} autoFocus={true}>
                    <ModalOverlay />
                    <ModalContent>
                        <ModalHeader>
                            <Text> Edit node </Text>
                        </ModalHeader>
                        <ModalBody>
                            <Stack w={"100%"}>
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
                                    placeholder="Title"
                                />
                                <Input
                                    value={mini_image}
                                    onChange={(e) => {
                                        setMini_image(e.target.value);
                                    }}
                                    placeholder="Image"
                                />
                                {type_ === "test" ? (
                                    <>
                                        <Input
                                            value={n_of_tests}
                                            placeholder="Number of tests"
                                            onChange={(e) => {
                                                setN_of_tests(
                                                    parseInt(e.target.value)
                                                );
                                            }}
                                            type="number"
                                        />
                                        <Box
                                            backgroundColor={"gray.100"}
                                            padding={"10px"}
                                            borderRadius={"10px"}
                                        >
                                            {Array.from(
                                                { length: n_of_tests as any },
                                                (_, i) => (
                                                    <Select
                                                        marginY={"5px"}
                                                        borderColor={"gray.300"}
                                                        value={ids[i]}
                                                        onChange={(e) => {
                                                            setIds([
                                                                ...ids.slice(
                                                                    0,
                                                                    i
                                                                ),
                                                                e.target.value,
                                                                ...ids.slice(
                                                                    i + 1
                                                                ),
                                                            ]);
                                                        }}
                                                    >
                                                        {ids_for_select.map(
                                                            (id) => (
                                                                <option value={id}>
                                                                    {id}
                                                                </option>
                                                            )
                                                        )}
                                                    </Select>
                                                )
                                            )}
                                        </Box>
                                    </>
                                ) : (
                                    <Box
                                        backgroundColor={"gray.100"}
                                        padding={"10px"}
                                        borderRadius={"10px"}
                                    >
                                        <Select
                                            marginY={"5px"}
                                            borderColor={"gray.300"}
                                            value={ids[0]}
                                            onChange={(e) => {
                                                setIds([e.target.value]);
                                            }}
                                        >
                                            {ids_for_select.map((id) => (
                                                <option value={id}>{id}</option>
                                            ))}
                                        </Select>
                                    </Box>
                                )}
                                {/*
                                  TODO: display n_of_tests * Input count 
                                 */}
                            </Stack>
                        </ModalBody>
                        <ModalFooter>
                            <Button colorScheme="blue" mr={3} onClick={onClose}>
                                Close
                            </Button>
                        </ModalFooter>
                    </ModalContent>
                </Modal>
            </>
        );
    }
}
