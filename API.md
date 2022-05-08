get existing sessions - "/sessions" GET {
    pass
} 

get sessions' groups - "/groups" GET {
    session
}

get group's members - "/members" GET {
    session
    group
}




