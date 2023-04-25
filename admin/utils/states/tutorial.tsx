import {create} from 'zustand';

export type TutorialState = {
    _id: string;
    title: string;
    theme: string;
    contentLevels: any;

    // setters
    setTutorialId: (id: string) => void;
    setTitle: (title: string) => void;
    setTheme: (theme: string) => void;
    setContentLevels: (contentLevels: any) => void;

    // getters
    getTutorialId: () => string | null;
    getTitle: () => string | null;
    getTheme: () => string | null;
    getContentLevels: () => any | null;

    // reset
    reset: () => void;

    // update
    update: (id: string, title: string, theme: string, contentLevels: any) => void;

    // sortContent, function for sort contentLevels, if it is not sorted
    sortContent: () => void;

    // addLevel, function for add new level to contentLevels
    addLevel: () => void;

    // deleteLevel, function for delete level from contentLevels
    deleteLevel: (index: number) => void;

    // addNode, function for add new node to level
    addNode: (levelIndex: number) => void;

    // deleteNode, function for delete node from level
    deleteNode: (levelIndex: number, nodeIndex: number) => void;

    // editNode, function for edit node in level
    editNode: (levelIndex: number, nodeIndex: number, content_type: string, content: string) => void;
}

const useTutorialStore = create((set, get: any) => ({
    _id: "",
    title: "",
    theme: "",
    contentLevels: [],

    // setters
    setTutorialId: (id: string) => set({ _id: id }),
    setTitle: (title: string) => set({ title: title }),
    setTheme: (theme: string) => set({ theme: theme }),
    setContentLevels: (contentLevels: any) => set({ contentLevels: contentLevels }),

    // getters
    getTutorialId: () => get()._id,
    getTitle: () => get().title,
    getTheme: () => get().theme,
    getContentLevels: () => get().contentLevels,

    // reset
    reset: () => set({ _id: "", title: "", theme: "", contentLevels: [] }),

    // update
    update: (id: string, title: string, theme: string, contentLevels: any) => set({ _id: id, title: title, theme: theme, contentLevels: contentLevels }),

    // sortContent, function for sort contentLevels, if it is not sorted
    sortContent: () => {
        // get current contentLevels
        const currentContentLevels = get().contentLevels;

        // sort contentLevels
        const sortedContentLevels = currentContentLevels.sort((a: any, b: any) => a[0] - b[0]);

        // update contentLevels
        set({contentLevels: sortedContentLevels});
    },

    //* function for work with contentLevels
    // add level
    addLevel: () => {
        // level looks like [1, [node1, node2, node3]]
        // node is {index: number, content_type: string, data: string, onDelete: () => {}}

        // get current contentLevels
        const currentContentLevels = get().contentLevels;

        // add level with one empty node to currentContentLevels
        const level = [currentContentLevels.length, [{index: 0, content_type: "text", data: "", onDelete: () => {}}]];

        // update contentLevels
        // contentLevels is array of levels
        set({contentLevels: [...currentContentLevels, level]});
    },

    // delete level
    deleteLevel: (index: number) => {
        // get current contentLevels
        const currentContentLevels = get().contentLevels;

        // remove level with index from contentLevels
        const filteredContentLevels = currentContentLevels.filter((level: any) => level[0] !== index);

        // update contentLevels
        set({contentLevels: filteredContentLevels});

        // reindex levels
        // after delete level, we need to reindex levels
        // for example, if we have 3 levels, and we delete level with index 1
        // we will have 2 levels with indexes 0 and 2 before reindexing
        // after reindexing we will have 2 levels with indexes 0 and 1

        // get current contentLevels
        const currentContentLevelsAfterDelete = get().contentLevels;

        // reindex levels
        const reindexedContentLevels = currentContentLevelsAfterDelete.map((level: any, index: number) => {
            return [index, level[1]];
        });

        // update contentLevels
        set({contentLevels: reindexedContentLevels});
    },

    // add node
    addNode: (levelIndex: number) => {
        // get current contentLevels
        const currentContentLevels = get().contentLevels;

        // get current level
        const currentLevel = currentContentLevels[levelIndex];

        // get last node index
        const lastNodeIndex = currentLevel[1][currentLevel[1].length - 1].index;

        // create new node
        const newNode = {index: lastNodeIndex + 1, content_type: "text", data: "", onDelete: () => {}};

        // add new node to current level
        currentLevel[1].push(newNode);

        // update contentLevels
        set({contentLevels: currentContentLevels});
    },

    // edit node
    editNode: (levelIndex: number, nodeIndex: number, type: string, data: string) => {
        // get current contentLevels
        const currentContentLevels = get().contentLevels;
        const currentLevel = currentContentLevels[levelIndex];
        const currentNode = currentLevel[1].find((node: any) => node.index === nodeIndex);
        currentNode.content_type = type;
        currentNode.data = data;
        set({contentLevels: currentContentLevels});
    },

    // delete node
    deleteNode: (levelIndex: number, nodeIndex: number) => {
        // get current contentLevels
        const currentContentLevels = get().contentLevels;

        // get current level
        const currentLevel = currentContentLevels[levelIndex];

        // filter current level
        const filteredLevel = currentLevel[1].filter((node: any) => node.index !== nodeIndex);

        // update current level
        currentLevel[1] = filteredLevel;

        // update contentLevels
        set({contentLevels: currentContentLevels});
    }

    // TODO: Functions for work with api

}));

export default useTutorialStore;