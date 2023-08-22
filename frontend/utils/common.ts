
// 统一处理显示用户名的逻辑
export function showUsername(username: string, principalId: string): string {
    if (username) {
        const MAX_LENGTH = 15; // 显示的最长字符
        if (username.length >= MAX_LENGTH) {
            return username.substring(0, MAX_LENGTH - 3) + '...';
        }
        return username;
    }
    return principalId ? principalId.substring(0, 5).toUpperCase() + '...' : '';
}
