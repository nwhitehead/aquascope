export interface CharPos {
  line: number;
  column: number;
}

export interface FilenameIndex {
  private_use_as_methods_instead: number;
}

export interface CharRange {
  start: CharPos;
  end: CharPos;
  filename: FilenameIndex;
}

export type MPathSegment =
  | { type: "Field"; value: number }
  | { type: "Index"; value: number }
  | { type: "Subslice"; value: [number, number] };

export type MMemorySegment =
  | { type: "Stack"; value: { frame: number; local: string } }
  | { type: "Heap"; value: { index: number } };

export interface MPath {
  segment: MMemorySegment;
  parts: Array<MPathSegment>;
}

export type MHeapAllocKind =
  | { type: "String"; value: { len: bigint } }
  | { type: "Vec"; value: { len: bigint } }
  | { type: "Box" };

export type Abbreviated<T> =
  | { type: "All"; value: Array<T> }
  | { type: "Only"; value: [Array<T>, T] };

export type MValue =
  | { type: "Bool"; value: boolean }
  | { type: "Char"; value: number }
  | { type: "Uint"; value: bigint }
  | { type: "Int"; value: bigint }
  | { type: "Float"; value: number }
  | { type: "Tuple"; value: Array<MValue> }
  | { type: "Array"; value: Abbreviated<MValue> }
  | {
      type: "Adt";
      value: {
        name: string;
        variant: string | null;
        fields: Array<[string, MValue]>;
        alloc_kind: MHeapAllocKind | null;
      };
    }
  | { type: "Pointer"; value: { path: MPath; range: bigint | null } }
  | { type: "Unallocated"; value: { alloc_id: number | null } };

export interface MLocal {
  name: string;
  value: MValue;
  moved_paths: Array<Array<MPathSegment>>;
}

export interface MFrame<L> {
  name: string;
  body_span: CharRange;
  location: L;
  locals: Array<MLocal>;
}

export interface MStack<L> {
  frames: Array<MFrame<L>>;
}

export interface MHeap {
  locations: Array<MValue>;
}

export interface MStep<L> {
  stack: MStack<L>;
  heap: MHeap;
}

export type MUndefinedBehavior =
  | { type: "PointerUseAfterFree"; value: { alloc_id: number } }
  | { type: "Other"; value: string };

export type MResult =
  | { type: "Success" }
  | { type: "Error"; value: MUndefinedBehavior };

export interface MTrace<L> {
  steps: Array<MStep<L>>;
  result: MResult;
}
