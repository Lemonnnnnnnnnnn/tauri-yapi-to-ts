export type ProjectBaseInfo = {
    _id: number,
    name: string
}

export type CategoryMenuList = {
    _id: number,
    name: string,
    interfaces?: CategoryDataList
}[]

export type CategoryDataList = {
    count: number,
    total: number,
    list: InterfaceDataItem[]
}

export type InterfaceDataItem = {
    _id: number,
    catid: number,
    title: string,
    path: string,
    project_id?: number;
    // ts: string
}

export type QueueLog = {
    msg: string,
    processd_number: number,
    is_success: boolean,
    resolved_interface: ResolvedInterface
}

export type ResolvedInterface = {
    interface: InterfaceData,
    ts_string: string,
}

export type InterfaceData = {
    _id: number,
    path: string,
    project_id: number,
    title: string,
    catid: number,
    method: string,
}