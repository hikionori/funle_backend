import { useRouter } from "next/router";
import { useCourseStore } from "../../utils/states/cource";
import Head from "next/head";
import {
  AbsoluteCenter,
  Box,
  Flex,
  Input,
  Select,
  Textarea,
} from "@chakra-ui/react";
import AddOptionButton from "../../components/addOptionButton";
import { useEffect, useState } from "react";
import CourseNode from "../../components/node/courseNode";
import CourceLevel from "../../components/levelNode/courseLevel";
import CourceLevelList from "../../components/levelNodeList/courseList";
import BottomFloatingButton from "../../components/bottomFloatingButton";
import { FaPlus } from "react-icons/fa";

export default function CreateNewCource() {
  const router = useRouter();

  const { title, setTitle } = useCourseStore((state: any) => ({
    title: state.title,
    setTitle: state.setTitle,
  }));
  const { description, setDescription } = useCourseStore((state: any) => ({
    description: state.description,
    setDescription: state.setDescription,
  }));
  const { levels, setLevels } = useCourseStore((state: any) => ({
    levels: state.levels,
    setLevels: state.setLevels,
  }));

  const { addLevel } = useCourseStore((state: any) => ({
    addLevel: state.addLevel,
  }));

  const { createCourse } = useCourseStore((state: any) => ({
    createCourse: state.createCourse,
  }));

  const [type_, setType] = useState("info");
  const [n_of_tests, setN_of_tests] = useState(1);

  return (
    <>
      <Head>
        <title>Create new cource</title>
      </Head>
      <main>
        <AbsoluteCenter width={"600px"}>
          <Input
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            border={"1px solid black"}
            placeholder={"Title"}
            focusBorderColor="orange.500"
            _hover={{
              borderColor: "orange.400",
              bgColor: "orange.50",
              _placeholder: { color: "blackAlpha.900" },
            }}
            marginY={"10px"}
          />
          <Textarea
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            border={"1px solid black"}
            placeholder={"Description"}
            focusBorderColor="orange.500"
            _hover={{
              borderColor: "orange.400",
              bgColor: "orange.50",
              _placeholder: { color: "blackAlpha.900" },
            }}
            marginBottom={"10px"}
          />
          <Box
            h={"80%"}
            overflowY={"scroll"}
          >
            <CourceLevelList />
          </Box>
          <AddOptionButton
            onClick={() => {
              addLevel();
            }}
          />
        </AbsoluteCenter>
        <BottomFloatingButton
          onClick={() => {
            createCourse();
            router.push("/cources");
          }}
          icon={<FaPlus />}
          text="Create"
        />
      </main>
    </>
  );
}
