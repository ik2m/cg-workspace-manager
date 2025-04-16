type Dir = {
  type: "dir";
  name: string;
  path: string;
  children: (Dir | File)[];
};

type File = {
  type: "file";
  name: string;
  path: string;
};

type PathTree = Dir;

export type { PathTree, File, Dir };
