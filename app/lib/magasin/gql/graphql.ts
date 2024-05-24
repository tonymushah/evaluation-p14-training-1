/* eslint-disable */
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  /**
   * A UUID is a unique 128-bit number, stored as 16 octets. UUIDs are parsed as
   * Strings within GraphQL. UUIDs are used to assign unique identifiers to
   * entities without requiring a central allocating authority.
   *
   * # References
   *
   * * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
   * * [RFC4122: A Universally Unique IDentifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)
   */
  UUID: { input: any; output: any; }
};

export type CrudMutations = {
  __typename?: 'CrudMutations';
  marque: MarqueCrudMutations;
};

export type MagasinMutations = {
  __typename?: 'MagasinMutations';
  marque: CrudMutations;
};

export type MagasinQueries = {
  __typename?: 'MagasinQueries';
  hello: Scalars['String']['output'];
};

export type Marque = {
  __typename?: 'Marque';
  designation: Scalars['String']['output'];
  idMarque: Scalars['UUID']['output'];
};

export type MarqueCrudMutations = {
  __typename?: 'MarqueCrudMutations';
  delete: Marque;
  deleteBatch: Array<Marque>;
  upsert: Marque;
  upsertBatch: Array<Marque>;
};


export type MarqueCrudMutationsDeleteArgs = {
  id: Scalars['UUID']['input'];
};


export type MarqueCrudMutationsDeleteBatchArgs = {
  ids: Array<Scalars['UUID']['input']>;
};


export type MarqueCrudMutationsUpsertArgs = {
  input: MarqueInput;
};


export type MarqueCrudMutationsUpsertBatchArgs = {
  input: Array<MarqueInput>;
};

export type MarqueInput = {
  designation: Scalars['String']['input'];
  idMarque?: Scalars['UUID']['input'];
};
