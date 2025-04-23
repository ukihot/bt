import { BreadType } from "../bt.types";

/*
export function findMatchingBread(
    input: string | undefined,
): BreadType | undefined {
    if (!input) return undefined;

    for (const key in BreadType) {
        const enumValue = BreadType[key as keyof typeof BreadType];
        if (enumValue.toLowerCase().startsWith(input)) {
            return enumValue as BreadType;
        }
    }

    return undefined;
}
*/

type TrieNodeType = {
    children: Map<string, TrieNodeType>;
    value: BreadType | null;
};

const TrieNode = (): TrieNodeType => ({
    children: new Map<string, TrieNodeType>(),
    value: null,
});

const Trie = () => {
    const root = TrieNode();

    const insert = (word: string, value: BreadType) => {
        let node = root;
        for (const char of word) {
            if (!node.children.has(char)) {
                node.children.set(char, TrieNode());
            }
            const nextNode = node.children.get(char);
            if (!nextNode) return;
            node = nextNode;
        }
        node.value = value;
    };

    const search = (prefix: string): BreadType | undefined => {
        let node = root;
        for (const char of prefix) {
            const nextNode = node.children.get(char);
            if (!nextNode) return undefined;
            node = nextNode;
            if (!node) return undefined;
        }
        return findFirstValue(node);
    };

    const findFirstValue = (
        node: ReturnType<typeof TrieNode>,
    ): BreadType | undefined => {
        if (node.value) return node.value;
        for (const child of node.children.values()) {
            const result = findFirstValue(child);
            if (result) return result;
        }
        return undefined;
    };

    return { insert, search };
};

const breadTrie = Trie();
for (const key in BreadType) {
    const enumValue = BreadType[key as keyof typeof BreadType];
    breadTrie.insert(enumValue.toLowerCase(), enumValue);
}

export function findMatchingBread(
    input: string | undefined,
): BreadType | undefined {
    if (!input) return undefined;
    return breadTrie.search(input.toLowerCase());
}
