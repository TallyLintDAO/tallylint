import type { UserInfo } from '@/types/user';

// 本地保存用户信息，没有网络访问时也可以显示
export const setUserInfoStorage = (user: UserInfo): void => {
    if(user.owner!==""){
        localStorage.setItem(`USER_${user.owner.toUpperCase()}`, JSON.stringify(user));
    }
};
// get方法注意缓存清没清
export const getUserInfoStorage = (principal: string): UserInfo | null => {
    const info = localStorage.getItem(`USER_${principal.toUpperCase()}`);
    if (null == info) return null;
    try {
        const read = JSON.parse(info) as UserInfo;
        return read;
    } catch (e) {
        console.error(`read user ${principal} info failed:`, e);
    }
    return null;
};

export const deleteUserInfoStorage = (principal: string): void => {
    console.log("deleteUser",principal)
    localStorage.removeItem(`USER_${principal.toUpperCase()}`);
};
