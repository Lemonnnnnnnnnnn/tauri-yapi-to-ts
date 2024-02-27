export interface Config {
    source_path?: string
    base_url?: string
    rate_limit?: number
    types_path?: string
    types_full_path?: string,
    break_seconds?: number,
    project_list?: ProjectList[],
    request_path?: string,
    request_template?: string,
    header_template?: string
    file_name_template?: string
    type_import_template?: string
    proxy?:string
}


export interface ProjectList {
    token: string
    project_id: string
    categories: CategoryType[]
}

export interface CategoryType {
    id: string
    name: string
    interfaces: InterfaceType[]
}

export interface InterfaceType {
    id: string
    name?: string
}

export interface SuccessResponse<T> {
    message: string,
    data: T
}

export interface QueueStatus {
    total:number;
    // success:number;
    // fail:number;
}

export interface TypesTree {
    full_path: string
    name: string
    children: TypesTree[]
}