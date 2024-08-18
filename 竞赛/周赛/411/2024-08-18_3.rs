struct Solution;

impl Solution {
    pub fn largest_palindrome(n: i32, k: i32) -> String {
        let mut res = Vec::new();
        match k {
            1 | 3 | 9 => {
                for _ in 0..n {
                    res.push(9 + b'0');
                }
            },
            2 => {
                for i in 0..n {
                    if i == 0 || i == n - 1 {
                        res.push(8 + b'0');
                    } else {
                        res.push(9 + b'0');
                    }
                }
            },
            4 => {
                for i in 0..n {
                    if i == 0 || i == 1 || i == n - 1 || i == n - 2 {
                        res.push(8 + b'0');
                    } else {
                        res.push(9 + b'0');
                    }
                }
            },
            5 => {
                for i in 0..n {
                    if i == 0 || i == n - 1 {
                        res.push(5 + b'0');
                    } else {
                        res.push(9 + b'0');
                    }
                }
            },
            6 => {
                if n == 1 {
                    res.push(6 + b'0');
                } else if n == 2 {
                    res.push(6 + b'0');
                    res.push(6 + b'0');
                } else if n == 3 {
                    res.push(8 + b'0');
                    res.push(8 + b'0');
                    res.push(8 + b'0');
                } else {
                    if n % 2 == 0 {
                        for i in 0..n {
                            if i == 0 || i == n - 1 {
                                res.push(8 + b'0');
                            } else if (i + 1) * 2 == n || i * 2 == n {
                                res.push(7 + b'0');
                            } else {
                                res.push(9 + b'0');
                            }
                        }
                    } else {
                        for i in 0..n {
                            if i == 0 || i == n - 1 || i * 2 + 1 == n {
                                res.push(8 + b'0');
                            } else {
                                res.push(9 + b'0');
                            }
                        }
                    }
                }
            },
            7 => {
                if n == 1 {
                    res.push(7 + b'0');
                } else if n == 2 {
                    res.push(7 + b'0');
                    res.push(7 + b'0');
                } else {
                    if n % 6 == 0 {
                        for _ in 0..n {
                            res.push(9 + b'0');
                        }
                    } else {
                        let mut i = 9;
                        while i > 0 {
                            let mut num = 0;
                            if n % 2 == 0 {
                                for j in 0..n {
                                    if (j + 1) * 2 == n || j * 2 == n {
                                        num = (num * 10 + i) % 7;
                                    } else {
                                        num = (num * 10 + 9) % 7;
                                    }
                                }
                            } else {
                                for j in 0..n {
                                    if j * 2 + 1 == n {
                                        num = (num * 10 + i) % 7;
                                    } else {
                                        num = (num * 10 + 9) % 7;
                                    }
                                }
                            }
                            if num == 0 {
                                if n % 2 == 0 {
                                    for j in 0..n {
                                        if (j + 1) * 2 == n || j * 2 == n {
                                            res.push(i + b'0');
                                        } else {
                                            res.push(9 + b'0');
                                        }
                                    }
                                } else {
                                    for j in 0..n {
                                        if j * 2 + 1 == n {
                                            res.push(i + b'0');
                                        } else {
                                            res.push(9 + b'0');
                                        }
                                    }
                                }
                                break;
                            }
                            i -= 1;
                        }
                    }
                }
            },
            8 => {
                for i in 0..n {
                    if i == 0 || i == 1 || i == 2 || i == n - 1 || i == n - 2 || i == n - 3 {
                        res.push(8 + b'0');
                    } else {
                        res.push(9 + b'0');
                    }
                }
            },
            _ => ()
        }
        unsafe {String::from_utf8_unchecked(res)}
    }
}
