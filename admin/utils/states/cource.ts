import { v4 as uuid } from "uuid";

import create from "zustand";
import { createCource, getCourceById, updateCource } from "../admin-sdk";

/*
    JSON:
    {
        _id: {
            $oid: string
        },
        title: string,
        description: string,
        levels: [
            [
                level index: number,
                [ array of nodes
                    { node
                        index: number for delete function,
                        id: string,
                        title: string,
                        mini_image: string,
                        type_ : string, // info | test
                        n_of_tests: number | string, // if type_ == "test" else null
                    }
                ]
            ]
        ]
    }
*/

export const useCourseStore = create((set, get: any) => ({
  _id: "",
  title: "",
  description: "",
  levels: [],

  // setters
  setCourseId: (id: string) => set({ _id: id }),
  setTitle: (title: string) => set({ title }),
  setDescription: (description: string) => set({ description }),
  setLevels: (levels: any[]) => set({ levels }),

  // getters
  getCourseId: () => get()._id,
  getTitle: () => get().title,
  getDescription: () => get().description,
  getLevels: () => get().levels,

  reset: () =>
    set({
      _id: "",
      title: "",
      description: "",
      levels: [],
    }),

  //sort Levels
  sortLevels: () => {
    const levels = get().levels;
    levels.sort((a: any, b: any) => a[0] - b[0]);
    set({ levels: levels });
  },

  addLevel: () => {
    const levels = get().levels;
    let id = uuid();
    const level = [
      levels.length,
      [
        {
          index: 0,
          id: id,
          ids: [],
          title: "",
          mini_image: "",
          type_: "test",
          n_of_tests: 1,
        },
      ],
    ];
    levels.push(level);
    set({ levels: levels });
  },

  deleteLevel: (index: number) => {
    const levels = get().levels;
    const filteredLevels = levels.filter((level: any) => level[0] !== index);

    // reindexing
    const reindexedLevels = filteredLevels.map((level: any, index: number) => {
      return [index, level[1]];
    });

    set({ levels: reindexedLevels });
  },

  addNode: (levelIndex: number) => {
    const levels = get().levels;
    const level = levels[levelIndex];
    const nodes = level[1][level[1].length - 1].index;
    let id = uuid();
    const newNode = {
      index: nodes + 1,
      id: id,
      ids: [],
      title: "",
      mini_image: "",
      type_: "test",
      n_of_tests: 1,
    };
    level[1].push(newNode);

    set({ levels: levels });
  },

  editNode: (
    levelIndex: number,
    nodeIndex: number,
    ids: string[],
    title: string,
    mini_image: string,
    type_: string,
    n_of_tests: number | null
  ) => {
    const levels = get().levels;
    const level = levels[levelIndex];
    const node = level[1][nodeIndex];
    node.ids = ids;
    node.title = title;
    node.mini_image = mini_image;
    node.type_ = type_;
    node.n_of_tests = n_of_tests;

    level[1][nodeIndex] = node;
    levels[levelIndex] = level;

    set({ levels: levels });
  },

  deleteNode: (levelIndex: number, nodeIndex: number) => {
    const levels = get().levels;
    const level = levels[levelIndex];
    const filteredLevel = level[1].filter(
      (node: any) => node.index !== nodeIndex
    );
    level[1] = filteredLevel;
    set({ levels: levels });
  },

  // TODO: API interaction
  createCourse: async () => {
    const title = get().title;
    const description = get().description;
    const levels = get().levels;

    const data = {
      title,
      description,
      levels,
    };

    await createCource(data);

    get().reset();
  },

  getCourse: async (id: string) => {
    const json = await getCourceById(id);

    set({
      _id: json._id.$oid,
      title: json.title,
      description: json.description,
      levels: json.levels,
    });
  },

  editCourse: async () => {
    const json = {
      title: get().title,
      description: get().description,
      levels: get().levels,
    };

    await updateCource(get()._id, json);
  },
}));
