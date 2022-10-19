//! Generates a random name.
//! 随机生成一个名字
use crate::pick_one;

const SURNAME: [&str; 100] = [
    "Smith",
    "Johnson",
    "Williams",
    "Brown",
    "Jones",
    "Garcia",
    "Miller",
    "Davis",
    "Rodriguez",
    "Martinez",
    "Hernandez",
    "Lopez",
    "Gonzales",
    "Wilson",
    "Anderson",
    "Thomas",
    "Taylor",
    "Moore",
    "Jackson",
    "Martin",
    "Lee",
    "Perez",
    "Thompson",
    "White",
    "Harris",
    "Sanchez",
    "Clark",
    "Ramirez",
    "Lewis",
    "Robinson",
    "Walker",
    "Young",
    "Allen",
    "King",
    "Wright",
    "Scott",
    "Torres",
    "Nguyen",
    "Hill",
    "Flores",
    "Green",
    "Adams",
    "Nelson",
    "Baker",
    "Hall",
    "Rivera",
    "Campbell",
    "Mitchell",
    "Carter",
    "Roberts",
    "Gomez",
    "Phillips",
    "Evans",
    "Turner",
    "Diaz",
    "Parker",
    "Cruz",
    "Edwards",
    "Collins",
    "Reyes",
    "Stewart",
    "Morris",
    "Morales",
    "Murphy",
    "Cook",
    "Rogers",
    "Gutierrez",
    "Ortiz",
    "Morgan",
    "Cooper",
    "Peterson",
    "Bailey",
    "Reed",
    "Kelly",
    "Howard",
    "Ramos",
    "Kim",
    "Cox",
    "Ward",
    "Richardson",
    "Watson",
    "Brooks",
    "Chavez",
    "Wood",
    "James",
    "Bennet",
    "Gray",
    "Mendoza",
    "Ruiz",
    "Hughes",
    "Price",
    "Alvarez",
    "Castillo",
    "Sanders",
    "Patel",
    "Myers",
    "Long",
    "Ross",
    "Foster",
    "Jimenez",
];
/// Generates a random surname.
/// 随机生成一个英文surname
///
/// # Example 示例
/// ```
/// use mock4rs::name::random_sur;
/// let  sur = random_sur();
/// println!("sur: {}", sur);
/// ```
pub fn random_sur() -> &'static str {
    pick_one(&SURNAME)
}
const GIVEN_NAME: [&str; 100] = [
    "James",
    "Robert",
    "John",
    "Michael",
    "David",
    "William",
    "Richard",
    "Joseph",
    "Thomas",
    "Charles",
    "Christopher",
    "Daniel",
    "Matthew",
    "Anthony",
    "Mark",
    "Donald",
    "Steven",
    "Paul",
    "Andrew",
    "Joshua",
    "Kenneth",
    "Kevin",
    "Brian",
    "George",
    "Timothy",
    "Ronald",
    "Edward",
    "Jason",
    "Jeffrey",
    "Ryan",
    "Jacob",
    "Gary",
    "Nicholas",
    "Eric",
    "Jonathan",
    "Stephen",
    "Larry",
    "Justin",
    "Scott",
    "Brandon",
    "Benjamin",
    "Samuel",
    "Gregory",
    "Alexander",
    "Frank",
    "Patrick",
    "Raymond",
    "Jack",
    "Dennis",
    "Jerry",
    "Tyler",
    "Aaron",
    "Jose",
    "Adam",
    "Nathan",
    "Henry",
    "Douglas",
    "Zachary",
    "Peter",
    "Kyle",
    "Ethan",
    "Walter",
    "Noah",
    "Jeremy",
    "Christian",
    "Keith",
    "Roger",
    "Terry",
    "Gerald",
    "Harold",
    "Sean",
    "Austin",
    "Carl",
    "Arthur",
    "Lawrence",
    "Dylan",
    "Jesse",
    "Jordan",
    "Bryan",
    "Billy",
    "Joe",
    "Bruce",
    "Gabriel",
    "Logan",
    "Albert",
    "Willie",
    "Alan",
    "Juan",
    "Wayne",
    "Elijah",
    "Randy",
    "Roy",
    "Vincent",
    "Ralph",
    "Eugene",
    "Russell",
    "Bobby",
    "Mason",
    "Philip",
    "Louis",
];
/// Generates a random given name.
/// 随机生成一个英文的given_name。
///
/// # Example 示例
/// ```
/// use mock4rs::name::random_given_name;
/// let  given_name = random_given_name();
/// println!("given_name: {}", given_name);
/// ```
pub fn random_given_name() -> &'static str {
    pick_one(&GIVEN_NAME)
}

/// Generates a random family name.
/// 随机生成一个英文名
///
/// # Example 示例
/// ```
/// use mock4rs::name::random_name;
/// let  string = random_name();
/// println!("string: {}", string);
/// ```
pub fn random_name() -> String {
    let sur = random_sur();
    let given = random_given_name();
    format!("{} {}", sur, given)
}

const C_SURNAME: [&str; 100] = [
    "王", "李", "张", "刘", "陈", "杨", "黄", "赵", "吴", "周", "徐", "孙", "马", "朱", "胡", "郭",
    "何", "高", "林", "郑", "谢", "罗", "梁", "宋", "唐", "许", "韩", "冯", "邓", "曹", "彭", "曾",
    "萧", "田", "董", "袁", "潘", "于", "蒋", "蔡", "余", "杜", "叶", "程", "苏", "魏", "吕", "丁",
    "任", "沈", "姚", "卢", "姜", "崔", "钟", "谭", "陆", "汪", "范", "金", "石", "廖", "贾", "夏",
    "韦", "付", "方", "白", "邹", "孟", "熊", "秦", "邱", "江", "尹", "薛", "闫", "段", "雷", "侯",
    "龙", "史", "陶", "黎", "贺", "顾", "毛", "郝", "龚", "邵", "万", "钱", "严", "覃", "武", "戴",
    "莫", "孔", "向", "汤",
];

/// Generates a random chinese surname name.
/// 随机生成一个中文姓
///
/// # Example 示例
/// ```
/// use mock4rs::name::random_c_surname;
/// let  c_surname = random_c_surname();
/// println!("c_surname: {}", c_surname);
/// ```
pub fn random_c_surname() -> &'static str {
    pick_one(&C_SURNAME)
}

const C_GIVEN_NAME: [&str; 100] = [
    "飞", "娜", "清", "鹏", "贤", "意", "烁", "浩", "帅", "强", "雪", "雷", "天", "超", "景", "灿",
    "材日", "振锐", "勇贤", "胤濡", "俊烁", "振烁", "谛逸", "泽轩", "诚材", "郁斌", "韦仕", "晨斌",
    "加祯", "铭逸", "良然", "良炳", "逸奇", "家泽", "峰禧", "加吉", "允文", "震良", "柔锋", "栋晖",
    "德暄", "运尧", "勇鹤", "杰鸿", "鹏远", "恒皓", "帆仕", "沛浩", "晓浩", "星家", "日吉", "宇翰",
    "延恒", "强礼", "斌帆", "福伟", "恒卓", "郁邦", "欣媛", "桃琬", "萱梅", "岚琛", "梅柏", "曦岚",
    "漫曦", "华依", "花橘", "春云", "霞敏", "冬香", "紫美", "雯珊", "楠静", "晨欣", "梦呈", "桂钰",
    "霞桂", "凡韵", "格冰", "凡桂", "娅桐", "岚采", "韵珠", "月华", "妮蕾", "函帆", "桂枫", "芳可",
    "芙紫", "寒璟", "惠琬", "玲彤", "采雨", "锦婧", "茜橘", "旭琳", "茹梦", "珍明", "茜紫", "蕾珊",
];
/// Generates a random chinese given name.
/// 随机生成一个中文的名
///
/// # Example 示例
/// ```
/// use mock4rs::name::random_c_given_name;
/// let  c_given_name = random_c_given_name();
/// println!("c_given_name: {}", c_given_name);
/// ```
pub fn random_c_given_name() -> &'static str {
    pick_one(&C_GIVEN_NAME)
}

/// Generates a random chinese family name.
/// 随机生成一个中文的姓名
///
/// # Example 示例
/// ```
/// use mock4rs::name::random_c_name;
/// let  c_name = random_c_name();
/// println!("c_name: {}", c_name);
/// ```
pub fn random_c_name() -> String {
    let sur = random_c_surname();
    let given = random_c_given_name();
    format!("{} {}", sur, given)
}
