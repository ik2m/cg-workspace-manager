type Dir = {
    type: 'dir';
    name: string;
    path: string;
    children: PathTree[];
}

type File = {
    type: 'file';
    name: string;
    path: string;
}

type PathTree = File | Dir;


export type { PathTree };