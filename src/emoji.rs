#[derive(Debug, Clone, Copy)]
pub struct Emoji {
    pub glyph: &'static str,
    /// Searchable keywords (lowercase).
    pub name: &'static str,
    /// Category shown in the tab row.
    pub cat: &'static str,
}

pub const EMOJIS: &[Emoji] = &[

    // Status
    Emoji { glyph: "✅", name: "check done ok yes success", cat: "Status" },
    Emoji { glyph: "❌", name: "cross no fail error", cat: "Status" },
    Emoji { glyph: "⚠️", name: "warning caution alert", cat: "Status" },
    Emoji { glyph: "⭐", name: "star favorite", cat: "Status" },
    Emoji { glyph: "❤️", name: "heart love", cat: "Status" },
    Emoji { glyph: "🔥", name: "fire hot lit", cat: "Status" },
    Emoji { glyph: "🚀", name: "rocket launch ship deploy", cat: "Status" },
    Emoji { glyph: "👍", name: "thumbs up like approve", cat: "Status" },
    Emoji { glyph: "👎", name: "thumbs down dislike", cat: "Status" },
    Emoji { glyph: "💡", name: "idea bulb light", cat: "Status" },
    Emoji { glyph: "🐛", name: "bug issue", cat: "Status" },
    Emoji { glyph: "✨", name: "sparkles new magic", cat: "Status" },
    Emoji { glyph: "🎉", name: "party tada celebrate", cat: "Status" },
    Emoji { glyph: "📌", name: "pin pushpin", cat: "Status" },
    Emoji { glyph: "📎", name: "paperclip attach", cat: "Status" },
    Emoji { glyph: "🔒", name: "lock secure private", cat: "Status" },
    Emoji { glyph: "🔑", name: "key", cat: "Status" },
    Emoji { glyph: "⚡", name: "bolt lightning fast zap", cat: "Status" },
    Emoji { glyph: "☕", name: "coffee break", cat: "Status" },
    Emoji { glyph: "🎯", name: "target goal", cat: "Status" },
    Emoji { glyph: "📝", name: "memo note write", cat: "Status" },
    Emoji { glyph: "🔍", name: "search magnifier find", cat: "Status" },
    Emoji { glyph: "🧪", name: "test experiment", cat: "Status" },
    Emoji { glyph: "📦", name: "package box release", cat: "Status" },
    Emoji { glyph: "⚙️", name: "gear settings config", cat: "Status" },
    Emoji { glyph: "🔧", name: "wrench fix tool", cat: "Status" },
    Emoji { glyph: "🚧", name: "construction wip", cat: "Status" },
    Emoji { glyph: "💬", name: "speech comment chat", cat: "Status" },
    Emoji { glyph: "📅", name: "calendar date", cat: "Status" },
    Emoji { glyph: "⏰", name: "alarm clock time", cat: "Status" },
    Emoji { glyph: "👀", name: "eyes look review", cat: "Status" },
    Emoji { glyph: "🙏", name: "pray thanks please", cat: "Status" },

    // Faces
    Emoji { glyph: "😀", name: "smile happy grin", cat: "Faces" },
    Emoji { glyph: "😃", name: "smiley happy cheerful", cat: "Faces" },
    Emoji { glyph: "😄", name: "grin smile happy", cat: "Faces" },
    Emoji { glyph: "😁", name: "beaming grin", cat: "Faces" },
    Emoji { glyph: "😆", name: "laughing squint", cat: "Faces" },
    Emoji { glyph: "😅", name: "sweat smile phew", cat: "Faces" },
    Emoji { glyph: "😂", name: "joy laugh lol", cat: "Faces" },
    Emoji { glyph: "🤣", name: "rolling floor laughing rofl", cat: "Faces" },
    Emoji { glyph: "😊", name: "blush smile happy", cat: "Faces" },
    Emoji { glyph: "😇", name: "angel innocent halo", cat: "Faces" },
    Emoji { glyph: "🙂", name: "slight smile", cat: "Faces" },
    Emoji { glyph: "🙃", name: "upside down silly", cat: "Faces" },
    Emoji { glyph: "😉", name: "wink", cat: "Faces" },
    Emoji { glyph: "😍", name: "love eyes heart", cat: "Faces" },
    Emoji { glyph: "🥰", name: "hearts affection love", cat: "Faces" },
    Emoji { glyph: "😘", name: "kiss love", cat: "Faces" },
    Emoji { glyph: "😋", name: "yum delicious tongue", cat: "Faces" },
    Emoji { glyph: "😎", name: "cool sunglasses", cat: "Faces" },
    Emoji { glyph: "🤓", name: "nerd glasses geek", cat: "Faces" },
    Emoji { glyph: "🤔", name: "thinking hmm", cat: "Faces" },
    Emoji { glyph: "🫡", name: "salute respect", cat: "Faces" },
    Emoji { glyph: "🤫", name: "shush quiet", cat: "Faces" },
    Emoji { glyph: "🤨", name: "suspicious raised eyebrow", cat: "Faces" },
    Emoji { glyph: "😐", name: "neutral face", cat: "Faces" },
    Emoji { glyph: "😑", name: "expressionless bored", cat: "Faces" },
    Emoji { glyph: "😶", name: "silent speechless", cat: "Faces" },
    Emoji { glyph: "🙄", name: "rolling eyes annoyed", cat: "Faces" },
    Emoji { glyph: "😏", name: "smirk", cat: "Faces" },
    Emoji { glyph: "😴", name: "sleep tired zzz", cat: "Faces" },
    Emoji { glyph: "🥱", name: "yawn sleepy", cat: "Faces" },
    Emoji { glyph: "😢", name: "cry sad", cat: "Faces" },
    Emoji { glyph: "😭", name: "sob crying tears", cat: "Faces" },
    Emoji { glyph: "😡", name: "angry mad", cat: "Faces" },
    Emoji { glyph: "🤬", name: "swearing angry", cat: "Faces" },
    Emoji { glyph: "🥳", name: "party birthday celebrate", cat: "Faces" },
    Emoji { glyph: "🥺", name: "pleading puppy eyes", cat: "Faces" },
    Emoji { glyph: "😱", name: "scream shock fear", cat: "Faces" },
    Emoji { glyph: "🤯", name: "mind blown shocked", cat: "Faces" },
    Emoji { glyph: "🤗", name: "hug", cat: "Faces" },
    Emoji { glyph: "🫠", name: "melting overwhelmed", cat: "Faces" },

    // Hands
    Emoji { glyph: "👋", name: "wave hello goodbye", cat: "Hands" },
    Emoji { glyph: "🤚", name: "raised hand stop", cat: "Hands" },
    Emoji { glyph: "✋", name: "hand stop high five", cat: "Hands" },
    Emoji { glyph: "🖐️", name: "open hand fingers", cat: "Hands" },
    Emoji { glyph: "👌", name: "ok perfect", cat: "Hands" },
    Emoji { glyph: "🤌", name: "pinched fingers", cat: "Hands" },
    Emoji { glyph: "🤏", name: "small tiny", cat: "Hands" },
    Emoji { glyph: "✌️", name: "peace victory", cat: "Hands" },
    Emoji { glyph: "🤞", name: "fingers crossed luck", cat: "Hands" },
    Emoji { glyph: "🤟", name: "love you hand", cat: "Hands" },
    Emoji { glyph: "🤘", name: "rock on metal", cat: "Hands" },
    Emoji { glyph: "🤙", name: "call me", cat: "Hands" },
    Emoji { glyph: "👈", name: "point left", cat: "Hands" },
    Emoji { glyph: "👉", name: "point right", cat: "Hands" },
    Emoji { glyph: "👆", name: "point up", cat: "Hands" },
    Emoji { glyph: "👇", name: "point down", cat: "Hands" },
    Emoji { glyph: "☝️", name: "index finger up", cat: "Hands" },
    Emoji { glyph: "👏", name: "clap applause", cat: "Hands" },
    Emoji { glyph: "🙌", name: "raised hands praise", cat: "Hands" },
    Emoji { glyph: "👐", name: "open hands", cat: "Hands" },
    Emoji { glyph: "🤝", name: "handshake deal agreement", cat: "Hands" },
    Emoji { glyph: "💪", name: "muscle strong flex", cat: "Hands" },
    Emoji { glyph: "🫶", name: "heart hands love", cat: "Hands" },

    // Animals
    Emoji { glyph: "🐶", name: "dog puppy", cat: "Animals" },
    Emoji { glyph: "🐱", name: "cat kitten", cat: "Animals" },
    Emoji { glyph: "🐭", name: "mouse", cat: "Animals" },
    Emoji { glyph: "🐹", name: "hamster", cat: "Animals" },
    Emoji { glyph: "🐰", name: "rabbit bunny", cat: "Animals" },
    Emoji { glyph: "🦊", name: "fox", cat: "Animals" },
    Emoji { glyph: "🐻", name: "bear", cat: "Animals" },
    Emoji { glyph: "🐼", name: "panda", cat: "Animals" },
    Emoji { glyph: "🐨", name: "koala", cat: "Animals" },
    Emoji { glyph: "🐯", name: "tiger", cat: "Animals" },
    Emoji { glyph: "🦁", name: "lion", cat: "Animals" },
    Emoji { glyph: "🐮", name: "cow", cat: "Animals" },
    Emoji { glyph: "🐷", name: "pig", cat: "Animals" },
    Emoji { glyph: "🐸", name: "frog", cat: "Animals" },
    Emoji { glyph: "🐵", name: "monkey", cat: "Animals" },
    Emoji { glyph: "🐧", name: "penguin", cat: "Animals" },
    Emoji { glyph: "🐦", name: "bird", cat: "Animals" },
    Emoji { glyph: "🦆", name: "duck", cat: "Animals" },
    Emoji { glyph: "🦉", name: "owl", cat: "Animals" },
    Emoji { glyph: "🦇", name: "bat", cat: "Animals" },
    Emoji { glyph: "🐺", name: "wolf", cat: "Animals" },
    Emoji { glyph: "🐗", name: "boar", cat: "Animals" },
    Emoji { glyph: "🐴", name: "horse", cat: "Animals" },
    Emoji { glyph: "🦄", name: "unicorn magic", cat: "Animals" },
    Emoji { glyph: "🐝", name: "bee honey", cat: "Animals" },
    Emoji { glyph: "🦋", name: "butterfly", cat: "Animals" },
    Emoji { glyph: "🐌", name: "snail", cat: "Animals" },
    Emoji { glyph: "🐞", name: "ladybug", cat: "Animals" },
    Emoji { glyph: "🐢", name: "turtle", cat: "Animals" },
    Emoji { glyph: "🐍", name: "snake", cat: "Animals" },
    Emoji { glyph: "🦖", name: "dinosaur t rex", cat: "Animals" },
    Emoji { glyph: "🐙", name: "octopus", cat: "Animals" },
    Emoji { glyph: "🦀", name: "crab", cat: "Animals" },
    Emoji { glyph: "🐬", name: "dolphin", cat: "Animals" },
    Emoji { glyph: "🐳", name: "whale", cat: "Animals" },
    Emoji { glyph: "🦈", name: "shark", cat: "Animals" },
    Emoji { glyph: "🦭", name: "seal", cat: "Animals" },
    Emoji { glyph: "🐘", name: "elephant", cat: "Animals" },
    Emoji { glyph: "🦒", name: "giraffe", cat: "Animals" },
    Emoji { glyph: "🦓", name: "zebra", cat: "Animals" },
    Emoji { glyph: "🦍", name: "gorilla", cat: "Animals" },

    // Nature
    Emoji { glyph: "🌱", name: "seedling plant grow", cat: "Nature" },
    Emoji { glyph: "🌲", name: "evergreen tree", cat: "Nature" },
    Emoji { glyph: "🌳", name: "tree nature", cat: "Nature" },
    Emoji { glyph: "🌴", name: "palm tree tropical", cat: "Nature" },
    Emoji { glyph: "🌵", name: "cactus desert", cat: "Nature" },
    Emoji { glyph: "🌷", name: "tulip flower", cat: "Nature" },
    Emoji { glyph: "🌹", name: "rose flower love", cat: "Nature" },
    Emoji { glyph: "🌻", name: "sunflower", cat: "Nature" },
    Emoji { glyph: "🌼", name: "blossom flower", cat: "Nature" },
    Emoji { glyph: "🌸", name: "cherry blossom", cat: "Nature" },
    Emoji { glyph: "🍀", name: "four leaf clover luck", cat: "Nature" },
    Emoji { glyph: "🍁", name: "maple leaf autumn", cat: "Nature" },
    Emoji { glyph: "🍂", name: "fallen leaves autumn", cat: "Nature" },
    Emoji { glyph: "🍃", name: "leaves wind", cat: "Nature" },
    Emoji { glyph: "🌍", name: "earth globe world", cat: "Nature" },
    Emoji { glyph: "🌙", name: "moon night", cat: "Nature" },
    Emoji { glyph: "🌈", name: "rainbow", cat: "Nature" },
    Emoji { glyph: "☀️", name: "sun sunny", cat: "Nature" },
    Emoji { glyph: "🌤️", name: "sun cloud", cat: "Nature" },
    Emoji { glyph: "⛅", name: "cloud sun", cat: "Nature" },
    Emoji { glyph: "🌧️", name: "rain rainy", cat: "Nature" },
    Emoji { glyph: "⛈️", name: "thunder storm", cat: "Nature" },
    Emoji { glyph: "❄️", name: "snow snowflake winter", cat: "Nature" },
    Emoji { glyph: "☃️", name: "snowman winter", cat: "Nature" },
    Emoji { glyph: "🌊", name: "wave ocean sea", cat: "Nature" },

    // Food
    Emoji { glyph: "🍎", name: "apple fruit", cat: "Food" },
    Emoji { glyph: "🍊", name: "orange fruit", cat: "Food" },
    Emoji { glyph: "🍋", name: "lemon", cat: "Food" },
    Emoji { glyph: "🍌", name: "banana", cat: "Food" },
    Emoji { glyph: "🍉", name: "watermelon", cat: "Food" },
    Emoji { glyph: "🍇", name: "grapes", cat: "Food" },
    Emoji { glyph: "🍓", name: "strawberry", cat: "Food" },
    Emoji { glyph: "🫐", name: "blueberry", cat: "Food" },
    Emoji { glyph: "🍒", name: "cherries", cat: "Food" },
    Emoji { glyph: "🍑", name: "peach", cat: "Food" },
    Emoji { glyph: "🥭", name: "mango", cat: "Food" },
    Emoji { glyph: "🍍", name: "pineapple", cat: "Food" },
    Emoji { glyph: "🥥", name: "coconut", cat: "Food" },
    Emoji { glyph: "🥑", name: "avocado", cat: "Food" },
    Emoji { glyph: "🍆", name: "eggplant aubergine", cat: "Food" },
    Emoji { glyph: "🥕", name: "carrot", cat: "Food" },
    Emoji { glyph: "🌽", name: "corn", cat: "Food" },
    Emoji { glyph: "🌶️", name: "chili pepper hot", cat: "Food" },
    Emoji { glyph: "🍕", name: "pizza", cat: "Food" },
    Emoji { glyph: "🍔", name: "burger hamburger", cat: "Food" },
    Emoji { glyph: "🍟", name: "fries", cat: "Food" },
    Emoji { glyph: "🌭", name: "hotdog", cat: "Food" },
    Emoji { glyph: "🍿", name: "popcorn", cat: "Food" },
    Emoji { glyph: "🧂", name: "salt", cat: "Food" },
    Emoji { glyph: "🍩", name: "donut doughnut", cat: "Food" },
    Emoji { glyph: "🍪", name: "cookie", cat: "Food" },
    Emoji { glyph: "🎂", name: "birthday cake", cat: "Food" },
    Emoji { glyph: "🍰", name: "cake dessert", cat: "Food" },
    Emoji { glyph: "🍫", name: "chocolate", cat: "Food" },
    Emoji { glyph: "🍬", name: "candy sweet", cat: "Food" },
    Emoji { glyph: "🍭", name: "lollipop", cat: "Food" },
    Emoji { glyph: "🍵", name: "tea", cat: "Food" },
    Emoji { glyph: "🧋", name: "bubble tea boba", cat: "Food" },
    Emoji { glyph: "🥤", name: "cup drink soda", cat: "Food" },
    Emoji { glyph: "🧃", name: "juice box", cat: "Food" },

    // Activities
    Emoji { glyph: "⚽", name: "soccer football", cat: "Activities" },
    Emoji { glyph: "🏀", name: "basketball", cat: "Activities" },
    Emoji { glyph: "🏈", name: "american football", cat: "Activities" },
    Emoji { glyph: "⚾", name: "baseball", cat: "Activities" },
    Emoji { glyph: "🥎", name: "softball", cat: "Activities" },
    Emoji { glyph: "🎾", name: "tennis", cat: "Activities" },
    Emoji { glyph: "🏐", name: "volleyball", cat: "Activities" },
    Emoji { glyph: "🏉", name: "rugby", cat: "Activities" },
    Emoji { glyph: "🥏", name: "frisbee", cat: "Activities" },
    Emoji { glyph: "🎱", name: "billiards pool", cat: "Activities" },
    Emoji { glyph: "🏓", name: "ping pong table tennis", cat: "Activities" },
    Emoji { glyph: "🏸", name: "badminton", cat: "Activities" },
    Emoji { glyph: "🥊", name: "boxing", cat: "Activities" },
    Emoji { glyph: "🥋", name: "martial arts", cat: "Activities" },
    Emoji { glyph: "🎮", name: "video game controller", cat: "Activities" },
    Emoji { glyph: "🕹️", name: "joystick arcade", cat: "Activities" },
    Emoji { glyph: "🎨", name: "art palette paint", cat: "Activities" },
    Emoji { glyph: "🎬", name: "movie clapper", cat: "Activities" },
    Emoji { glyph: "🎤", name: "microphone karaoke", cat: "Activities" },
    Emoji { glyph: "🎧", name: "headphones music", cat: "Activities" },
    Emoji { glyph: "🎸", name: "guitar", cat: "Activities" },
    Emoji { glyph: "🎹", name: "piano keyboard", cat: "Activities" },
    Emoji { glyph: "🥁", name: "drum", cat: "Activities" },
    Emoji { glyph: "🎻", name: "violin", cat: "Activities" },
    Emoji { glyph: "🏆", name: "trophy winner", cat: "Activities" },
    Emoji { glyph: "🥇", name: "gold medal first", cat: "Activities" },
    Emoji { glyph: "🥈", name: "silver medal second", cat: "Activities" },
    Emoji { glyph: "🥉", name: "bronze medal third", cat: "Activities" },
    Emoji { glyph: "🎖️", name: "medal honor", cat: "Activities" },

    // Travel
    Emoji { glyph: "🚗", name: "car automobile", cat: "Travel" },
    Emoji { glyph: "🚕", name: "taxi cab", cat: "Travel" },
    Emoji { glyph: "🚌", name: "bus", cat: "Travel" },
    Emoji { glyph: "🚎", name: "trolleybus", cat: "Travel" },
    Emoji { glyph: "🏎️", name: "racing car", cat: "Travel" },
    Emoji { glyph: "🚓", name: "police car", cat: "Travel" },
    Emoji { glyph: "🚑", name: "ambulance", cat: "Travel" },
    Emoji { glyph: "🚒", name: "fire truck", cat: "Travel" },
    Emoji { glyph: "🚚", name: "truck delivery", cat: "Travel" },
    Emoji { glyph: "🚲", name: "bicycle bike", cat: "Travel" },
    Emoji { glyph: "🛵", name: "scooter", cat: "Travel" },
    Emoji { glyph: "🏍️", name: "motorcycle", cat: "Travel" },
    Emoji { glyph: "✈️", name: "airplane flight", cat: "Travel" },
    Emoji { glyph: "🛸", name: "ufo alien", cat: "Travel" },
    Emoji { glyph: "🚁", name: "helicopter", cat: "Travel" },
    Emoji { glyph: "🚢", name: "ship boat", cat: "Travel" },
    Emoji { glyph: "⛵", name: "sailboat", cat: "Travel" },
    Emoji { glyph: "🚂", name: "train locomotive", cat: "Travel" },
    Emoji { glyph: "🚆", name: "train", cat: "Travel" },
    Emoji { glyph: "🚇", name: "metro subway", cat: "Travel" },
    Emoji { glyph: "🚉", name: "station", cat: "Travel" },
    Emoji { glyph: "🗽", name: "statue liberty", cat: "Travel" },
    Emoji { glyph: "🗼", name: "tokyo tower", cat: "Travel" },
    Emoji { glyph: "🏰", name: "castle", cat: "Travel" },
    Emoji { glyph: "🏯", name: "japanese castle", cat: "Travel" },
    Emoji { glyph: "🏝️", name: "island tropical", cat: "Travel" },
    Emoji { glyph: "🏖️", name: "beach", cat: "Travel" },
    Emoji { glyph: "🏕️", name: "camping tent", cat: "Travel" },
    Emoji { glyph: "🏔️", name: "mountain snow", cat: "Travel" },

    // Objects
    Emoji { glyph: "⌚", name: "watch time", cat: "Objects" },
    Emoji { glyph: "📱", name: "phone mobile", cat: "Objects" },
    Emoji { glyph: "💻", name: "laptop computer", cat: "Objects" },
    Emoji { glyph: "🖥️", name: "desktop computer monitor", cat: "Objects" },
    Emoji { glyph: "⌨️", name: "keyboard", cat: "Objects" },
    Emoji { glyph: "🖱️", name: "mouse computer", cat: "Objects" },
    Emoji { glyph: "🖨️", name: "printer", cat: "Objects" },
    Emoji { glyph: "💾", name: "floppy disk save", cat: "Objects" },
    Emoji { glyph: "💿", name: "cd disc", cat: "Objects" },
    Emoji { glyph: "📷", name: "camera photo", cat: "Objects" },
    Emoji { glyph: "📹", name: "video camera", cat: "Objects" },
    Emoji { glyph: "🎥", name: "movie camera", cat: "Objects" },
    Emoji { glyph: "📺", name: "television tv", cat: "Objects" },
    Emoji { glyph: "📻", name: "radio", cat: "Objects" },
    Emoji { glyph: "🔋", name: "battery", cat: "Objects" },
    Emoji { glyph: "🔌", name: "plug electric", cat: "Objects" },
    Emoji { glyph: "🔦", name: "flashlight torch", cat: "Objects" },
    Emoji { glyph: "🕯️", name: "candle", cat: "Objects" },
    Emoji { glyph: "📚", name: "books library", cat: "Objects" },
    Emoji { glyph: "📖", name: "open book reading", cat: "Objects" },
    Emoji { glyph: "📕", name: "closed book red", cat: "Objects" },
    Emoji { glyph: "📄", name: "document page", cat: "Objects" },
    Emoji { glyph: "📃", name: "page document", cat: "Objects" },
    Emoji { glyph: "📜", name: "scroll parchment", cat: "Objects" },
    Emoji { glyph: "📋", name: "clipboard", cat: "Objects" },
    Emoji { glyph: "📁", name: "folder directory", cat: "Objects" },
    Emoji { glyph: "📂", name: "open folder", cat: "Objects" },
    Emoji { glyph: "🗂️", name: "file dividers", cat: "Objects" },
    Emoji { glyph: "🗑️", name: "trash delete bin", cat: "Objects" },
    Emoji { glyph: "🔓", name: "unlock open lock", cat: "Objects" },
    Emoji { glyph: "🔨", name: "hammer", cat: "Objects" },
    Emoji { glyph: "🛠️", name: "tools repair", cat: "Objects" },
    Emoji { glyph: "🧰", name: "toolbox", cat: "Objects" },
    Emoji { glyph: "🧲", name: "magnet", cat: "Objects" },
    Emoji { glyph: "🧯", name: "fire extinguisher", cat: "Objects" },
    Emoji { glyph: "💎", name: "diamond gem", cat: "Objects" },
    Emoji { glyph: "💰", name: "money bag", cat: "Objects" },
    Emoji { glyph: "💳", name: "credit card payment", cat: "Objects" },
    Emoji { glyph: "✉️", name: "envelope email", cat: "Objects" },
    Emoji { glyph: "📧", name: "email e-mail", cat: "Objects" },
    Emoji { glyph: "🎁", name: "gift present", cat: "Objects" },
    Emoji { glyph: "🎈", name: "balloon", cat: "Objects" },
    Emoji { glyph: "🎀", name: "ribbon bow", cat: "Objects" },
    Emoji { glyph: "🧸", name: "teddy bear toy", cat: "Objects" },
    Emoji { glyph: "🪄", name: "magic wand", cat: "Objects" },

    // Symbols
    Emoji { glyph: "✔️", name: "check tick done", cat: "Symbols" },
    Emoji { glyph: "✖️", name: "cross multiply close", cat: "Symbols" },
    Emoji { glyph: "➕", name: "plus add", cat: "Symbols" },
    Emoji { glyph: "➖", name: "minus subtract", cat: "Symbols" },
    Emoji { glyph: "➗", name: "divide division", cat: "Symbols" },
    Emoji { glyph: "❓", name: "question help", cat: "Symbols" },
    Emoji { glyph: "❗", name: "exclamation important", cat: "Symbols" },
    Emoji { glyph: "‼️", name: "double exclamation", cat: "Symbols" },
    Emoji { glyph: "⁉️", name: "question exclamation", cat: "Symbols" },
    Emoji { glyph: "💯", name: "hundred perfect score", cat: "Symbols" },
    Emoji { glyph: "♻️", name: "recycle environment", cat: "Symbols" },
    Emoji { glyph: "🚫", name: "prohibited forbidden", cat: "Symbols" },
    Emoji { glyph: "⛔", name: "no entry stop", cat: "Symbols" },
    Emoji { glyph: "🔴", name: "red circle", cat: "Symbols" },
    Emoji { glyph: "🟠", name: "orange circle", cat: "Symbols" },
    Emoji { glyph: "🟡", name: "yellow circle", cat: "Symbols" },
    Emoji { glyph: "🟢", name: "green circle", cat: "Symbols" },
    Emoji { glyph: "🔵", name: "blue circle", cat: "Symbols" },
    Emoji { glyph: "🟣", name: "purple circle", cat: "Symbols" },
    Emoji { glyph: "⚫", name: "black circle", cat: "Symbols" },
    Emoji { glyph: "⚪", name: "white circle", cat: "Symbols" },
    Emoji { glyph: "🟥", name: "red square", cat: "Symbols" },
    Emoji { glyph: "🟧", name: "orange square", cat: "Symbols" },
    Emoji { glyph: "🟨", name: "yellow square", cat: "Symbols" },
    Emoji { glyph: "🟩", name: "green square", cat: "Symbols" },
    Emoji { glyph: "🟦", name: "blue square", cat: "Symbols" },
    Emoji { glyph: "🟪", name: "purple square", cat: "Symbols" },
    Emoji { glyph: "⬛", name: "black square", cat: "Symbols" },
    Emoji { glyph: "⬜", name: "white square", cat: "Symbols" },
    Emoji { glyph: "➡️", name: "arrow right", cat: "Symbols" },
    Emoji { glyph: "⬅️", name: "arrow left", cat: "Symbols" },
    Emoji { glyph: "⬆️", name: "arrow up", cat: "Symbols" },
    Emoji { glyph: "⬇️", name: "arrow down", cat: "Symbols" },
    Emoji { glyph: "↗️", name: "arrow northeast", cat: "Symbols" },
    Emoji { glyph: "↘️", name: "arrow southeast", cat: "Symbols" },
    Emoji { glyph: "↙️", name: "arrow southwest", cat: "Symbols" },
    Emoji { glyph: "↖️", name: "arrow northwest", cat: "Symbols" },
    Emoji { glyph: "🔄", name: "refresh reload rotate", cat: "Symbols" },
    Emoji { glyph: "🔁", name: "repeat loop", cat: "Symbols" },
    Emoji { glyph: "🔂", name: "repeat one", cat: "Symbols" },
    Emoji { glyph: "▶️", name: "play start", cat: "Symbols" },
    Emoji { glyph: "⏸️", name: "pause", cat: "Symbols" },
    Emoji { glyph: "⏹️", name: "stop", cat: "Symbols" },
    Emoji { glyph: "⏭️", name: "next skip", cat: "Symbols" },
    Emoji { glyph: "⏮️", name: "previous back", cat: "Symbols" },
    Emoji { glyph: "🔀", name: "shuffle random", cat: "Symbols" },
    Emoji { glyph: "🔔", name: "bell notification", cat: "Symbols" },
    Emoji { glyph: "🔕", name: "mute notifications", cat: "Symbols" },
    Emoji { glyph: "ℹ️", name: "info information", cat: "Symbols" },
    Emoji { glyph: "©️", name: "copyright", cat: "Symbols" },
    Emoji { glyph: "®️", name: "registered trademark", cat: "Symbols" },
    Emoji { glyph: "™️", name: "trademark", cat: "Symbols" },
    Emoji { glyph: "💤", name: "sleep zzz", cat: "Symbols" },
    Emoji { glyph: "♾️", name: "infinity endless", cat: "Symbols" },
    Emoji { glyph: "🔆", name: "brightness high", cat: "Symbols" },
    Emoji { glyph: "🔅", name: "brightness low", cat: "Symbols" },
    Emoji { glyph: "〰️", name: "wave dash", cat: "Symbols" },
    Emoji { glyph: "➰", name: "curly loop", cat: "Symbols" },
    Emoji { glyph: "➿", name: "double curly loop", cat: "Symbols" },
    Emoji { glyph: "⌛", name: "hourglass", cat: "Symbols" },
    Emoji { glyph: "⏏️", name: "eject symbol", cat: "Symbols" },
    Emoji { glyph: "⏩", name: "black right pointing double triangle", cat: "Symbols" },
    Emoji { glyph: "⏪", name: "black left pointing double triangle", cat: "Symbols" },
    Emoji { glyph: "⏫", name: "black up pointing double triangle", cat: "Symbols" },
    Emoji { glyph: "⏬", name: "black down pointing double triangle", cat: "Symbols" },
    Emoji { glyph: "⏯️", name: "black right pointing triangle with double vertical bar", cat: "Symbols" },

    // Objects
    Emoji { glyph: "⏱️", name: "stopwatch", cat: "Objects" },
    Emoji { glyph: "⏲️", name: "timer clock", cat: "Objects" },

    // Animals
    Emoji { glyph: "⏳", name: "hourglass with flowing sand", cat: "Animals" },

    // Symbols
    Emoji { glyph: "⏺️", name: "black circle for record", cat: "Symbols" },
    Emoji { glyph: "Ⓜ️", name: "circled latin capital letter m", cat: "Symbols" },
    Emoji { glyph: "▪️", name: "black small square", cat: "Symbols" },
    Emoji { glyph: "▫️", name: "white small square", cat: "Symbols" },
    Emoji { glyph: "◀️", name: "black left pointing triangle", cat: "Symbols" },
    Emoji { glyph: "◻️", name: "white medium square", cat: "Symbols" },
    Emoji { glyph: "◼️", name: "black medium square", cat: "Symbols" },
    Emoji { glyph: "◽", name: "white medium small square", cat: "Symbols" },
    Emoji { glyph: "◾", name: "black medium small square", cat: "Symbols" },

    // Nature
    Emoji { glyph: "☁️", name: "cloud", cat: "Nature" },

    // Objects
    Emoji { glyph: "☂️", name: "umbrella", cat: "Objects" },

    // Nature
    Emoji { glyph: "☄️", name: "comet", cat: "Nature" },

    // Objects
    Emoji { glyph: "☎️", name: "black telephone", cat: "Objects" },

    // Symbols
    Emoji { glyph: "☑️", name: "ballot box with check", cat: "Symbols" },

    // Nature
    Emoji { glyph: "☔", name: "umbrella with rain drops", cat: "Nature" },

    // Symbols
    Emoji { glyph: "☘️", name: "shamrock", cat: "Symbols" },
    Emoji { glyph: "☠️", name: "skull and crossbones", cat: "Symbols" },

    // Objects
    Emoji { glyph: "☢️", name: "radioactive sign", cat: "Objects" },
    Emoji { glyph: "☣️", name: "biohazard sign", cat: "Objects" },

    // Symbols
    Emoji { glyph: "☦️", name: "orthodox cross", cat: "Symbols" },

    // Nature
    Emoji { glyph: "☪️", name: "star and crescent", cat: "Nature" },

    // Symbols
    Emoji { glyph: "☮️", name: "peace symbol", cat: "Symbols" },
    Emoji { glyph: "☯️", name: "yin yang", cat: "Symbols" },

    // People
    Emoji { glyph: "☸️", name: "wheel of dharma", cat: "People" },
    Emoji { glyph: "☹️", name: "white frowning face", cat: "People" },
    Emoji { glyph: "☺️", name: "white smiling face", cat: "People" },

    // Objects
    Emoji { glyph: "♀️", name: "female sign", cat: "Objects" },
    Emoji { glyph: "♂️", name: "male sign", cat: "Objects" },

    // Symbols
    Emoji { glyph: "♈", name: "aries", cat: "Symbols" },
    Emoji { glyph: "♉", name: "taurus", cat: "Symbols" },
    Emoji { glyph: "♊", name: "gemini", cat: "Symbols" },
    Emoji { glyph: "♋", name: "cancer", cat: "Symbols" },
    Emoji { glyph: "♌", name: "leo", cat: "Symbols" },
    Emoji { glyph: "♍", name: "virgo", cat: "Symbols" },
    Emoji { glyph: "♎", name: "libra", cat: "Symbols" },
    Emoji { glyph: "♏", name: "scorpius", cat: "Symbols" },
    Emoji { glyph: "♐", name: "sagittarius", cat: "Symbols" },

    // Food
    Emoji { glyph: "♑", name: "capricorn", cat: "Food" },

    // Symbols
    Emoji { glyph: "♒", name: "aquarius", cat: "Symbols" },
    Emoji { glyph: "♓", name: "pisces", cat: "Symbols" },
    Emoji { glyph: "♟️", name: "black chess pawn", cat: "Symbols" },
    Emoji { glyph: "♠️", name: "black spade suit", cat: "Symbols" },
    Emoji { glyph: "♣️", name: "black club suit", cat: "Symbols" },

    // Activities
    Emoji { glyph: "♥️", name: "black heart suit", cat: "Activities" },

    // Objects
    Emoji { glyph: "♦️", name: "black diamond suit", cat: "Objects" },

    // Symbols
    Emoji { glyph: "♨️", name: "hot springs", cat: "Symbols" },

    // People
    Emoji { glyph: "♿", name: "wheelchair symbol", cat: "People" },

    // Objects
    Emoji { glyph: "⚒️", name: "hammer and pick", cat: "Objects" },

    // Symbols
    Emoji { glyph: "⚓", name: "anchor", cat: "Symbols" },
    Emoji { glyph: "⚔️", name: "crossed swords", cat: "Symbols" },
    Emoji { glyph: "⚕️", name: "staff of aesculapius", cat: "Symbols" },
    Emoji { glyph: "⚖️", name: "scales", cat: "Symbols" },
    Emoji { glyph: "⚗️", name: "alembic", cat: "Symbols" },
    Emoji { glyph: "⚛️", name: "atom symbol", cat: "Symbols" },
    Emoji { glyph: "⚜️", name: "fleur de lis", cat: "Symbols" },

    // Objects
    Emoji { glyph: "⚧️", name: "male with stroke and male and female sign", cat: "Objects" },

    // Symbols
    Emoji { glyph: "⚰️", name: "coffin", cat: "Symbols" },
    Emoji { glyph: "⚱️", name: "funeral urn", cat: "Symbols" },

    // People
    Emoji { glyph: "⛄", name: "snowman without snow", cat: "People" },

    // Symbols
    Emoji { glyph: "⛎", name: "ophiuchus", cat: "Symbols" },
    Emoji { glyph: "⛏️", name: "pick", cat: "Symbols" },
    Emoji { glyph: "⛑️", name: "helmet with white cross", cat: "Symbols" },
    Emoji { glyph: "⛓️", name: "chains", cat: "Symbols" },
    Emoji { glyph: "⛩️", name: "shinto shrine", cat: "Symbols" },
    Emoji { glyph: "⛪", name: "church", cat: "Symbols" },

    // Nature
    Emoji { glyph: "⛰️", name: "mountain", cat: "Nature" },

    // Objects
    Emoji { glyph: "⛱️", name: "umbrella on ground", cat: "Objects" },

    // Symbols
    Emoji { glyph: "⛲", name: "fountain", cat: "Symbols" },
    Emoji { glyph: "⛳", name: "flag in hole", cat: "Symbols" },

    // Travel
    Emoji { glyph: "⛴️", name: "ferry", cat: "Travel" },

    // Activities
    Emoji { glyph: "⛷️", name: "skier", cat: "Activities" },

    // Symbols
    Emoji { glyph: "⛸️", name: "ice skate", cat: "Symbols" },

    // People
    Emoji { glyph: "⛹️", name: "person with ball", cat: "People" },

    // Symbols
    Emoji { glyph: "⛺", name: "tent", cat: "Symbols" },

    // Travel
    Emoji { glyph: "⛽", name: "fuel pump", cat: "Travel" },

    // Symbols
    Emoji { glyph: "✂️", name: "black scissors", cat: "Symbols" },

    // People
    Emoji { glyph: "✊", name: "raised fist", cat: "People" },
    Emoji { glyph: "✍️", name: "writing hand", cat: "People" },

    // Symbols
    Emoji { glyph: "✏️", name: "pencil", cat: "Symbols" },
    Emoji { glyph: "✒️", name: "black nib", cat: "Symbols" },
    Emoji { glyph: "✝️", name: "latin cross", cat: "Symbols" },

    // Nature
    Emoji { glyph: "✡️", name: "star of david", cat: "Nature" },

    // Symbols
    Emoji { glyph: "✳️", name: "eight spoked asterisk", cat: "Symbols" },

    // Nature
    Emoji { glyph: "✴️", name: "eight pointed black star", cat: "Nature" },

    // Symbols
    Emoji { glyph: "❇️", name: "sparkle", cat: "Symbols" },
    Emoji { glyph: "❎", name: "negative squared cross mark", cat: "Symbols" },
    Emoji { glyph: "❔", name: "white question mark ornament", cat: "Symbols" },
    Emoji { glyph: "❕", name: "white exclamation mark ornament", cat: "Symbols" },

    // Activities
    Emoji { glyph: "❣️", name: "heavy heart exclamation mark ornament", cat: "Activities" },

    // Symbols
    Emoji { glyph: "⤴️", name: "arrow pointing rightwards then curving upwards", cat: "Symbols" },
    Emoji { glyph: "⤵️", name: "arrow pointing rightwards then curving downwards", cat: "Symbols" },
    Emoji { glyph: "⭕", name: "heavy large circle", cat: "Symbols" },

    // Activities
    Emoji { glyph: "〽️", name: "part alternation mark", cat: "Activities" },

    // Animals
    Emoji { glyph: "㊗️", name: "circled ideograph congratulation", cat: "Animals" },

    // Symbols
    Emoji { glyph: "㊙️", name: "circled ideograph secret", cat: "Symbols" },
    Emoji { glyph: "🀄", name: "mahjong tile red dragon", cat: "Symbols" },

    // Travel
    Emoji { glyph: "🃏", name: "playing card black joker", cat: "Travel" },

    // Symbols
    Emoji { glyph: "🅰️", name: "negative squared latin capital letter a", cat: "Symbols" },
    Emoji { glyph: "🅱️", name: "negative squared latin capital letter b", cat: "Symbols" },
    Emoji { glyph: "🅾️", name: "negative squared latin capital letter o", cat: "Symbols" },
    Emoji { glyph: "🅿️", name: "negative squared latin capital letter p", cat: "Symbols" },
    Emoji { glyph: "🆎", name: "negative squared ab", cat: "Symbols" },
    Emoji { glyph: "🆑", name: "squared cl", cat: "Symbols" },
    Emoji { glyph: "🆒", name: "squared cool", cat: "Symbols" },
    Emoji { glyph: "🆓", name: "squared free", cat: "Symbols" },
    Emoji { glyph: "🆔", name: "squared id", cat: "Symbols" },
    Emoji { glyph: "🆕", name: "squared new", cat: "Symbols" },
    Emoji { glyph: "🆖", name: "squared ng", cat: "Symbols" },
    Emoji { glyph: "🆗", name: "squared ok", cat: "Symbols" },
    Emoji { glyph: "🆘", name: "squared sos", cat: "Symbols" },
    Emoji { glyph: "🆙", name: "squared up with exclamation mark", cat: "Symbols" },
    Emoji { glyph: "🆚", name: "squared vs", cat: "Symbols" },
    Emoji { glyph: "🈁", name: "squared katakana koko", cat: "Symbols" },
    Emoji { glyph: "🈂️", name: "squared katakana sa", cat: "Symbols" },
    Emoji { glyph: "🈚", name: "squared cjk unified ideograph 7121", cat: "Symbols" },
    Emoji { glyph: "🈯", name: "squared cjk unified ideograph 6307", cat: "Symbols" },
    Emoji { glyph: "🈲", name: "squared cjk unified ideograph 7981", cat: "Symbols" },
    Emoji { glyph: "🈳", name: "squared cjk unified ideograph 7a7a", cat: "Symbols" },
    Emoji { glyph: "🈴", name: "squared cjk unified ideograph 5408", cat: "Symbols" },
    Emoji { glyph: "🈵", name: "squared cjk unified ideograph 6e80", cat: "Symbols" },
    Emoji { glyph: "🈶", name: "squared cjk unified ideograph 6709", cat: "Symbols" },
    Emoji { glyph: "🈷️", name: "squared cjk unified ideograph 6708", cat: "Symbols" },
    Emoji { glyph: "🈸", name: "squared cjk unified ideograph 7533", cat: "Symbols" },
    Emoji { glyph: "🈹", name: "squared cjk unified ideograph 5272", cat: "Symbols" },
    Emoji { glyph: "🈺", name: "squared cjk unified ideograph 55b6", cat: "Symbols" },
    Emoji { glyph: "🉐", name: "circled ideograph advantage", cat: "Symbols" },
    Emoji { glyph: "🉑", name: "circled ideograph accept", cat: "Symbols" },
    Emoji { glyph: "🌀", name: "cyclone", cat: "Symbols" },
    Emoji { glyph: "🌁", name: "foggy", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🌂", name: "closed umbrella", cat: "Objects" },

    // Nature
    Emoji { glyph: "🌃", name: "night with stars", cat: "Nature" },
    Emoji { glyph: "🌄", name: "sunrise over mountains", cat: "Nature" },
    Emoji { glyph: "🌅", name: "sunrise", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🌆", name: "cityscape at dusk", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🌇", name: "sunset over buildings", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🌉", name: "bridge at night", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🌋", name: "volcano", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🌌", name: "milky way", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🌎", name: "earth globe americas", cat: "Nature" },
    Emoji { glyph: "🌏", name: "earth globe asia australia", cat: "Nature" },
    Emoji { glyph: "🌐", name: "globe with meridians", cat: "Nature" },
    Emoji { glyph: "🌑", name: "new moon symbol", cat: "Nature" },
    Emoji { glyph: "🌒", name: "waxing crescent moon symbol", cat: "Nature" },
    Emoji { glyph: "🌓", name: "first quarter moon symbol", cat: "Nature" },
    Emoji { glyph: "🌔", name: "waxing gibbous moon symbol", cat: "Nature" },
    Emoji { glyph: "🌕", name: "full moon symbol", cat: "Nature" },
    Emoji { glyph: "🌖", name: "waning gibbous moon symbol", cat: "Nature" },
    Emoji { glyph: "🌗", name: "last quarter moon symbol", cat: "Nature" },
    Emoji { glyph: "🌘", name: "waning crescent moon symbol", cat: "Nature" },

    // People
    Emoji { glyph: "🌚", name: "new moon with face", cat: "People" },
    Emoji { glyph: "🌛", name: "first quarter moon with face", cat: "People" },
    Emoji { glyph: "🌜", name: "last quarter moon with face", cat: "People" },
    Emoji { glyph: "🌝", name: "full moon with face", cat: "People" },
    Emoji { glyph: "🌞", name: "sun with face", cat: "People" },

    // Animals
    Emoji { glyph: "🌟", name: "glowing star", cat: "Animals" },

    // Nature
    Emoji { glyph: "🌠", name: "shooting star", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🌡️", name: "thermometer", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🌥️", name: "white sun behind cloud", cat: "Nature" },
    Emoji { glyph: "🌦️", name: "white sun behind cloud with rain", cat: "Nature" },
    Emoji { glyph: "🌨️", name: "cloud with snow", cat: "Nature" },
    Emoji { glyph: "🌩️", name: "cloud with lightning", cat: "Nature" },
    Emoji { glyph: "🌪️", name: "cloud with tornado", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🌫️", name: "fog", cat: "Symbols" },

    // People
    Emoji { glyph: "🌬️", name: "wind blowing face", cat: "People" },

    // Food
    Emoji { glyph: "🌮", name: "taco", cat: "Food" },
    Emoji { glyph: "🌯", name: "burrito", cat: "Food" },

    // Symbols
    Emoji { glyph: "🌰", name: "chestnut", cat: "Symbols" },
    Emoji { glyph: "🌺", name: "hibiscus", cat: "Symbols" },
    Emoji { glyph: "🌾", name: "ear of rice", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🌿", name: "herb", cat: "Nature" },
    Emoji { glyph: "🍄", name: "mushroom", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🍅", name: "tomato", cat: "Symbols" },

    // Food
    Emoji { glyph: "🍈", name: "melon", cat: "Food" },
    Emoji { glyph: "🍏", name: "green apple", cat: "Food" },
    Emoji { glyph: "🍐", name: "pear", cat: "Food" },
    Emoji { glyph: "🍖", name: "meat on bone", cat: "Food" },

    // People
    Emoji { glyph: "🍗", name: "poultry leg", cat: "People" },

    // Symbols
    Emoji { glyph: "🍘", name: "rice cracker", cat: "Symbols" },
    Emoji { glyph: "🍙", name: "rice ball", cat: "Symbols" },

    // People
    Emoji { glyph: "🍚", name: "cooked rice", cat: "People" },

    // Symbols
    Emoji { glyph: "🍛", name: "curry and rice", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🍜", name: "steaming bowl", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🍝", name: "spaghetti", cat: "Symbols" },

    // Food
    Emoji { glyph: "🍞", name: "bread", cat: "Food" },
    Emoji { glyph: "🍠", name: "roasted sweet potato", cat: "Food" },

    // Symbols
    Emoji { glyph: "🍡", name: "dango", cat: "Symbols" },
    Emoji { glyph: "🍢", name: "oden", cat: "Symbols" },

    // Food
    Emoji { glyph: "🍣", name: "sushi", cat: "Food" },

    // Animals
    Emoji { glyph: "🍤", name: "fried shrimp", cat: "Animals" },
    Emoji { glyph: "🍥", name: "fish cake with swirl design", cat: "Animals" },

    // Food
    Emoji { glyph: "🍦", name: "soft ice cream", cat: "Food" },

    // Symbols
    Emoji { glyph: "🍧", name: "shaved ice", cat: "Symbols" },

    // Food
    Emoji { glyph: "🍨", name: "ice cream", cat: "Food" },

    // Nature
    Emoji { glyph: "🍮", name: "custard", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🍯", name: "honey pot", cat: "Symbols" },
    Emoji { glyph: "🍱", name: "bento box", cat: "Symbols" },

    // Food
    Emoji { glyph: "🍲", name: "pot of food", cat: "Food" },

    // People
    Emoji { glyph: "🍳", name: "cooking", cat: "People" },

    // Symbols
    Emoji { glyph: "🍴", name: "fork and knife", cat: "Symbols" },

    // Food
    Emoji { glyph: "🍶", name: "sake bottle and cup", cat: "Food" },
    Emoji { glyph: "🍷", name: "wine glass", cat: "Food" },

    // Symbols
    Emoji { glyph: "🍸", name: "cocktail glass", cat: "Symbols" },

    // Food
    Emoji { glyph: "🍹", name: "tropical drink", cat: "Food" },

    // Animals
    Emoji { glyph: "🍺", name: "beer mug", cat: "Animals" },

    // People
    Emoji { glyph: "🍻", name: "clinking beer mugs", cat: "People" },
    Emoji { glyph: "🍼", name: "baby bottle", cat: "People" },

    // Symbols
    Emoji { glyph: "🍽️", name: "fork and knife with plate", cat: "Symbols" },

    // Food
    Emoji { glyph: "🍾", name: "bottle with popping cork", cat: "Food" },

    // Symbols
    Emoji { glyph: "🎃", name: "jack o lantern", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🎄", name: "christmas tree", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🎅", name: "father christmas", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🎆", name: "fireworks", cat: "Nature" },
    Emoji { glyph: "🎇", name: "firework sparkler", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🎊", name: "confetti ball", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🎋", name: "tanabata tree", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🎌", name: "crossed flags", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🎍", name: "pine decoration", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🎎", name: "japanese dolls", cat: "Symbols" },

    // Travel
    Emoji { glyph: "🎏", name: "carp streamer", cat: "Travel" },

    // Nature
    Emoji { glyph: "🎐", name: "wind chime", cat: "Nature" },

    // Animals
    Emoji { glyph: "🎑", name: "moon viewing ceremony", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🎒", name: "school satchel", cat: "Symbols" },
    Emoji { glyph: "🎓", name: "graduation cap", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🎗️", name: "reminder ribbon", cat: "Objects" },

    // Activities
    Emoji { glyph: "🎙️", name: "studio microphone", cat: "Activities" },

    // Symbols
    Emoji { glyph: "🎚️", name: "level slider", cat: "Symbols" },
    Emoji { glyph: "🎛️", name: "control knobs", cat: "Symbols" },
    Emoji { glyph: "🎞️", name: "film frames", cat: "Symbols" },
    Emoji { glyph: "🎟️", name: "admission tickets", cat: "Symbols" },

    // Travel
    Emoji { glyph: "🎠", name: "carousel horse", cat: "Travel" },

    // Symbols
    Emoji { glyph: "🎡", name: "ferris wheel", cat: "Symbols" },
    Emoji { glyph: "🎢", name: "roller coaster", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🎣", name: "fishing pole and fish", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🎦", name: "cinema", cat: "Symbols" },
    Emoji { glyph: "🎩", name: "top hat", cat: "Symbols" },

    // Activities
    Emoji { glyph: "🎪", name: "circus tent", cat: "Activities" },

    // Symbols
    Emoji { glyph: "🎫", name: "ticket", cat: "Symbols" },

    // Activities
    Emoji { glyph: "🎭", name: "performing arts", cat: "Activities" },

    // Symbols
    Emoji { glyph: "🎰", name: "slot machine", cat: "Symbols" },

    // Activities
    Emoji { glyph: "🎲", name: "game die", cat: "Activities" },

    // Animals
    Emoji { glyph: "🎳", name: "bowling", cat: "Animals" },

    // Nature
    Emoji { glyph: "🎴", name: "flower playing cards", cat: "Nature" },

    // Activities
    Emoji { glyph: "🎵", name: "musical note", cat: "Activities" },
    Emoji { glyph: "🎶", name: "multiple musical notes", cat: "Activities" },

    // Objects
    Emoji { glyph: "🎷", name: "saxophone", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🎺", name: "trumpet", cat: "Symbols" },

    // Activities
    Emoji { glyph: "🎼", name: "musical score", cat: "Activities" },

    // People
    Emoji { glyph: "🎽", name: "running shirt with sash", cat: "People" },

    // Activities
    Emoji { glyph: "🎿", name: "ski and ski boot", cat: "Activities" },

    // Symbols
    Emoji { glyph: "🏁", name: "chequered flag", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🏂", name: "snowboarder", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🏃", name: "runner", cat: "Symbols" },

    // Activities
    Emoji { glyph: "🏄", name: "surfer", cat: "Activities" },
    Emoji { glyph: "🏅", name: "sports medal", cat: "Activities" },

    // Symbols
    Emoji { glyph: "🏇", name: "horse racing", cat: "Symbols" },

    // Activities
    Emoji { glyph: "🏊", name: "swimmer", cat: "Activities" },

    // Symbols
    Emoji { glyph: "🏋️", name: "weight lifter", cat: "Symbols" },

    // Activities
    Emoji { glyph: "🏌️", name: "golfer", cat: "Activities" },

    // Animals
    Emoji { glyph: "🏏", name: "cricket bat and ball", cat: "Animals" },

    // Activities
    Emoji { glyph: "🏑", name: "field hockey stick and ball", cat: "Activities" },
    Emoji { glyph: "🏒", name: "ice hockey stick and puck", cat: "Activities" },

    // Objects
    Emoji { glyph: "🏗️", name: "building construction", cat: "Objects" },
    Emoji { glyph: "🏘️", name: "house buildings", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🏙️", name: "cityscape", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🏚️", name: "derelict house building", cat: "Objects" },
    Emoji { glyph: "🏛️", name: "classical building", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🏜️", name: "desert", cat: "Symbols" },
    Emoji { glyph: "🏞️", name: "national park", cat: "Symbols" },
    Emoji { glyph: "🏟️", name: "stadium", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🏠", name: "house building", cat: "Objects" },
    Emoji { glyph: "🏡", name: "house with garden", cat: "Objects" },
    Emoji { glyph: "🏢", name: "office building", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🏣", name: "japanese post office", cat: "Symbols" },
    Emoji { glyph: "🏤", name: "european post office", cat: "Symbols" },
    Emoji { glyph: "🏥", name: "hospital", cat: "Symbols" },
    Emoji { glyph: "🏦", name: "bank", cat: "Symbols" },
    Emoji { glyph: "🏧", name: "automated teller machine", cat: "Symbols" },
    Emoji { glyph: "🏨", name: "hotel", cat: "Symbols" },
    Emoji { glyph: "🏩", name: "love hotel", cat: "Symbols" },
    Emoji { glyph: "🏪", name: "convenience store", cat: "Symbols" },
    Emoji { glyph: "🏫", name: "school", cat: "Symbols" },

    // Activities
    Emoji { glyph: "🏬", name: "department store", cat: "Activities" },

    // Symbols
    Emoji { glyph: "🏭", name: "factory", cat: "Symbols" },
    Emoji { glyph: "🏮", name: "izakaya lantern", cat: "Symbols" },
    Emoji { glyph: "🏳️", name: "waving white flag", cat: "Symbols" },
    Emoji { glyph: "🏴", name: "waving black flag", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🏵️", name: "rosette", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🏷️", name: "label", cat: "Symbols" },
    Emoji { glyph: "🏹", name: "bow and arrow", cat: "Symbols" },
    Emoji { glyph: "🏺", name: "amphora", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🐀", name: "rat", cat: "Animals" },
    Emoji { glyph: "🐁", name: "mouse", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🐂", name: "ox", cat: "Symbols" },
    Emoji { glyph: "🐃", name: "water buffalo", cat: "Symbols" },
    Emoji { glyph: "🐄", name: "cow", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🐅", name: "tiger", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🐆", name: "leopard", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🐇", name: "rabbit", cat: "Animals" },
    Emoji { glyph: "🐈", name: "cat", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🐉", name: "dragon", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🐊", name: "crocodile", cat: "Animals" },
    Emoji { glyph: "🐋", name: "whale", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🐎", name: "horse", cat: "Symbols" },
    Emoji { glyph: "🐏", name: "ram", cat: "Symbols" },
    Emoji { glyph: "🐐", name: "goat", cat: "Symbols" },
    Emoji { glyph: "🐑", name: "sheep", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🐒", name: "monkey", cat: "Animals" },
    Emoji { glyph: "🐓", name: "rooster", cat: "Animals" },
    Emoji { glyph: "🐔", name: "chicken", cat: "Animals" },
    Emoji { glyph: "🐕", name: "dog", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🐖", name: "pig", cat: "Symbols" },
    Emoji { glyph: "🐚", name: "spiral shell", cat: "Symbols" },
    Emoji { glyph: "🐜", name: "ant", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🐟", name: "fish", cat: "Animals" },
    Emoji { glyph: "🐠", name: "tropical fish", cat: "Animals" },
    Emoji { glyph: "🐡", name: "blowfish", cat: "Animals" },
    Emoji { glyph: "🐣", name: "hatching chick", cat: "Animals" },

    // People
    Emoji { glyph: "🐤", name: "baby chick", cat: "People" },
    Emoji { glyph: "🐥", name: "front facing baby chick", cat: "People" },

    // Symbols
    Emoji { glyph: "🐩", name: "poodle", cat: "Symbols" },
    Emoji { glyph: "🐪", name: "dromedary camel", cat: "Symbols" },
    Emoji { glyph: "🐫", name: "bactrian camel", cat: "Symbols" },

    // People
    Emoji { glyph: "🐲", name: "dragon face", cat: "People" },
    Emoji { glyph: "🐽", name: "pig nose", cat: "People" },

    // Symbols
    Emoji { glyph: "🐾", name: "paw prints", cat: "Symbols" },
    Emoji { glyph: "🐿️", name: "chipmunk", cat: "Symbols" },

    // People
    Emoji { glyph: "👁️", name: "eye", cat: "People" },

    // Symbols
    Emoji { glyph: "👂", name: "ear", cat: "Symbols" },

    // People
    Emoji { glyph: "👃", name: "nose", cat: "People" },
    Emoji { glyph: "👄", name: "mouth", cat: "People" },
    Emoji { glyph: "👅", name: "tongue", cat: "People" },
    Emoji { glyph: "👊", name: "fisted hand sign", cat: "People" },

    // Objects
    Emoji { glyph: "👑", name: "crown", cat: "Objects" },

    // People
    Emoji { glyph: "👒", name: "womans hat", cat: "People" },
    Emoji { glyph: "👓", name: "eyeglasses", cat: "People" },

    // Symbols
    Emoji { glyph: "👔", name: "necktie", cat: "Symbols" },

    // Objects
    Emoji { glyph: "👕", name: "t shirt", cat: "Objects" },

    // Symbols
    Emoji { glyph: "👖", name: "jeans", cat: "Symbols" },
    Emoji { glyph: "👗", name: "dress", cat: "Symbols" },
    Emoji { glyph: "👘", name: "kimono", cat: "Symbols" },
    Emoji { glyph: "👙", name: "bikini", cat: "Symbols" },

    // People
    Emoji { glyph: "👚", name: "womans clothes", cat: "People" },

    // Symbols
    Emoji { glyph: "👛", name: "purse", cat: "Symbols" },

    // People
    Emoji { glyph: "👜", name: "handbag", cat: "People" },

    // Symbols
    Emoji { glyph: "👝", name: "pouch", cat: "Symbols" },

    // People
    Emoji { glyph: "👞", name: "mans shoe", cat: "People" },

    // Objects
    Emoji { glyph: "👟", name: "athletic shoe", cat: "Objects" },
    Emoji { glyph: "👠", name: "high heeled shoe", cat: "Objects" },

    // People
    Emoji { glyph: "👡", name: "womans sandal", cat: "People" },
    Emoji { glyph: "👢", name: "womans boots", cat: "People" },
    Emoji { glyph: "👣", name: "footprints", cat: "People" },

    // Travel
    Emoji { glyph: "👤", name: "bust in silhouette", cat: "Travel" },
    Emoji { glyph: "👥", name: "busts in silhouette", cat: "Travel" },

    // People
    Emoji { glyph: "👦", name: "boy", cat: "People" },
    Emoji { glyph: "👧", name: "girl", cat: "People" },
    Emoji { glyph: "👨", name: "man", cat: "People" },
    Emoji { glyph: "👩", name: "woman", cat: "People" },
    Emoji { glyph: "👪", name: "family", cat: "People" },
    Emoji { glyph: "👫", name: "man and woman holding hands", cat: "People" },
    Emoji { glyph: "👬", name: "two men holding hands", cat: "People" },
    Emoji { glyph: "👭", name: "two women holding hands", cat: "People" },
    Emoji { glyph: "👮", name: "police officer", cat: "People" },
    Emoji { glyph: "👯", name: "woman with bunny ears", cat: "People" },

    // Symbols
    Emoji { glyph: "👰", name: "bride with veil", cat: "Symbols" },

    // People
    Emoji { glyph: "👱", name: "person with blond hair", cat: "People" },
    Emoji { glyph: "👲", name: "man with gua pi mao", cat: "People" },
    Emoji { glyph: "👳", name: "man with turban", cat: "People" },
    Emoji { glyph: "👴", name: "older man", cat: "People" },
    Emoji { glyph: "👵", name: "older woman", cat: "People" },
    Emoji { glyph: "👶", name: "baby", cat: "People" },

    // Symbols
    Emoji { glyph: "👷", name: "construction worker", cat: "Symbols" },

    // People
    Emoji { glyph: "👸", name: "princess", cat: "People" },

    // Symbols
    Emoji { glyph: "👹", name: "japanese ogre", cat: "Symbols" },
    Emoji { glyph: "👺", name: "japanese goblin", cat: "Symbols" },
    Emoji { glyph: "👻", name: "ghost", cat: "Symbols" },

    // People
    Emoji { glyph: "👼", name: "baby angel", cat: "People" },

    // Animals
    Emoji { glyph: "👽", name: "extraterrestrial alien", cat: "Animals" },

    // Symbols
    Emoji { glyph: "👾", name: "alien monster", cat: "Symbols" },
    Emoji { glyph: "👿", name: "imp", cat: "Symbols" },
    Emoji { glyph: "💀", name: "skull", cat: "Symbols" },

    // People
    Emoji { glyph: "💁", name: "information desk person", cat: "People" },
    Emoji { glyph: "💂", name: "guardsman", cat: "People" },

    // Activities
    Emoji { glyph: "💃", name: "dancer", cat: "Activities" },

    // Symbols
    Emoji { glyph: "💄", name: "lipstick", cat: "Symbols" },
    Emoji { glyph: "💅", name: "nail polish", cat: "Symbols" },

    // People
    Emoji { glyph: "💆", name: "face massage", cat: "People" },

    // Symbols
    Emoji { glyph: "💇", name: "haircut", cat: "Symbols" },
    Emoji { glyph: "💈", name: "barber pole", cat: "Symbols" },
    Emoji { glyph: "💉", name: "syringe", cat: "Symbols" },
    Emoji { glyph: "💊", name: "pill", cat: "Symbols" },

    // People
    Emoji { glyph: "💋", name: "kiss mark", cat: "People" },

    // Symbols
    Emoji { glyph: "💌", name: "love letter", cat: "Symbols" },
    Emoji { glyph: "💍", name: "ring", cat: "Symbols" },

    // People
    Emoji { glyph: "💏", name: "kiss", cat: "People" },

    // Symbols
    Emoji { glyph: "💐", name: "bouquet", cat: "Symbols" },

    // Activities
    Emoji { glyph: "💑", name: "couple with heart", cat: "Activities" },

    // Symbols
    Emoji { glyph: "💒", name: "wedding", cat: "Symbols" },

    // Activities
    Emoji { glyph: "💓", name: "beating heart", cat: "Activities" },
    Emoji { glyph: "💔", name: "broken heart", cat: "Activities" },
    Emoji { glyph: "💕", name: "two hearts", cat: "Activities" },
    Emoji { glyph: "💖", name: "sparkling heart", cat: "Activities" },

    // Animals
    Emoji { glyph: "💗", name: "growing heart", cat: "Animals" },

    // Activities
    Emoji { glyph: "💘", name: "heart with arrow", cat: "Activities" },
    Emoji { glyph: "💙", name: "blue heart", cat: "Activities" },
    Emoji { glyph: "💚", name: "green heart", cat: "Activities" },
    Emoji { glyph: "💛", name: "yellow heart", cat: "Activities" },
    Emoji { glyph: "💜", name: "purple heart", cat: "Activities" },
    Emoji { glyph: "💝", name: "heart with ribbon", cat: "Activities" },
    Emoji { glyph: "💞", name: "revolving hearts", cat: "Activities" },

    // Animals
    Emoji { glyph: "💟", name: "heart decoration", cat: "Animals" },

    // Objects
    Emoji { glyph: "💠", name: "diamond shape with a dot inside", cat: "Objects" },

    // Symbols
    Emoji { glyph: "💢", name: "anger symbol", cat: "Symbols" },
    Emoji { glyph: "💣", name: "bomb", cat: "Symbols" },
    Emoji { glyph: "💥", name: "collision symbol", cat: "Symbols" },
    Emoji { glyph: "💦", name: "splashing sweat symbol", cat: "Symbols" },
    Emoji { glyph: "💧", name: "droplet", cat: "Symbols" },
    Emoji { glyph: "💨", name: "dash symbol", cat: "Symbols" },
    Emoji { glyph: "💩", name: "pile of poo", cat: "Symbols" },
    Emoji { glyph: "💫", name: "dizzy symbol", cat: "Symbols" },

    // Objects
    Emoji { glyph: "💭", name: "thought balloon", cat: "Objects" },

    // Nature
    Emoji { glyph: "💮", name: "white flower", cat: "Nature" },

    // Symbols
    Emoji { glyph: "💱", name: "currency exchange", cat: "Symbols" },

    // Objects
    Emoji { glyph: "💲", name: "heavy dollar sign", cat: "Objects" },
    Emoji { glyph: "💴", name: "banknote with yen sign", cat: "Objects" },
    Emoji { glyph: "💵", name: "banknote with dollar sign", cat: "Objects" },
    Emoji { glyph: "💶", name: "banknote with euro sign", cat: "Objects" },
    Emoji { glyph: "💷", name: "banknote with pound sign", cat: "Objects" },

    // Animals
    Emoji { glyph: "💸", name: "money with wings", cat: "Animals" },

    // Activities
    Emoji { glyph: "💹", name: "chart with upwards trend and yen sign", cat: "Activities" },

    // Symbols
    Emoji { glyph: "💺", name: "seat", cat: "Symbols" },
    Emoji { glyph: "💼", name: "briefcase", cat: "Symbols" },
    Emoji { glyph: "💽", name: "minidisc", cat: "Symbols" },
    Emoji { glyph: "📀", name: "dvd", cat: "Symbols" },

    // Food
    Emoji { glyph: "📆", name: "tear off calendar", cat: "Food" },

    // Travel
    Emoji { glyph: "📇", name: "card index", cat: "Travel" },

    // Activities
    Emoji { glyph: "📈", name: "chart with upwards trend", cat: "Activities" },
    Emoji { glyph: "📉", name: "chart with downwards trend", cat: "Activities" },
    Emoji { glyph: "📊", name: "bar chart", cat: "Activities" },

    // Symbols
    Emoji { glyph: "📍", name: "round pushpin", cat: "Symbols" },
    Emoji { glyph: "📏", name: "straight ruler", cat: "Symbols" },
    Emoji { glyph: "📐", name: "triangular ruler", cat: "Symbols" },

    // Objects
    Emoji { glyph: "📑", name: "bookmark tabs", cat: "Objects" },

    // Symbols
    Emoji { glyph: "📒", name: "ledger", cat: "Symbols" },

    // Objects
    Emoji { glyph: "📓", name: "notebook", cat: "Objects" },

    // Animals
    Emoji { glyph: "📔", name: "notebook with decorative cover", cat: "Animals" },

    // Objects
    Emoji { glyph: "📗", name: "green book", cat: "Objects" },
    Emoji { glyph: "📘", name: "blue book", cat: "Objects" },
    Emoji { glyph: "📙", name: "orange book", cat: "Objects" },

    // Symbols
    Emoji { glyph: "📛", name: "name badge", cat: "Symbols" },

    // Objects
    Emoji { glyph: "📞", name: "telephone receiver", cat: "Objects" },

    // Symbols
    Emoji { glyph: "📟", name: "pager", cat: "Symbols" },
    Emoji { glyph: "📠", name: "fax machine", cat: "Symbols" },
    Emoji { glyph: "📡", name: "satellite antenna", cat: "Symbols" },
    Emoji { glyph: "📢", name: "public address loudspeaker", cat: "Symbols" },

    // Objects
    Emoji { glyph: "📣", name: "cheering megaphone", cat: "Objects" },

    // Symbols
    Emoji { glyph: "📤", name: "outbox tray", cat: "Symbols" },
    Emoji { glyph: "📥", name: "inbox tray", cat: "Symbols" },

    // Objects
    Emoji { glyph: "📨", name: "incoming envelope", cat: "Objects" },
    Emoji { glyph: "📩", name: "envelope with downwards arrow above", cat: "Objects" },

    // Symbols
    Emoji { glyph: "📪", name: "closed mailbox with lowered flag", cat: "Symbols" },
    Emoji { glyph: "📫", name: "closed mailbox with raised flag", cat: "Symbols" },
    Emoji { glyph: "📬", name: "open mailbox with raised flag", cat: "Symbols" },
    Emoji { glyph: "📭", name: "open mailbox with lowered flag", cat: "Symbols" },
    Emoji { glyph: "📮", name: "postbox", cat: "Symbols" },
    Emoji { glyph: "📯", name: "postal horn", cat: "Symbols" },

    // Objects
    Emoji { glyph: "📰", name: "newspaper", cat: "Objects" },
    Emoji { glyph: "📲", name: "mobile phone with rightwards arrow at left", cat: "Objects" },

    // Animals
    Emoji { glyph: "📳", name: "vibration mode", cat: "Animals" },

    // Objects
    Emoji { glyph: "📴", name: "mobile phone off", cat: "Objects" },
    Emoji { glyph: "📵", name: "no mobile phones", cat: "Objects" },

    // Symbols
    Emoji { glyph: "📶", name: "antenna with bars", cat: "Symbols" },

    // Objects
    Emoji { glyph: "📸", name: "camera with flash", cat: "Objects" },
    Emoji { glyph: "📼", name: "videocassette", cat: "Objects" },

    // Symbols
    Emoji { glyph: "📽️", name: "film projector", cat: "Symbols" },
    Emoji { glyph: "📿", name: "prayer beads", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🔃", name: "clockwise downwards and upwards open circle arrows", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🔇", name: "speaker with cancellation stroke", cat: "Symbols" },
    Emoji { glyph: "🔈", name: "speaker", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🔉", name: "speaker with one sound wave", cat: "Nature" },
    Emoji { glyph: "🔊", name: "speaker with three sound waves", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🔎", name: "right pointing magnifying glass", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🔏", name: "lock with ink pen", cat: "Objects" },
    Emoji { glyph: "🔐", name: "closed lock with key", cat: "Objects" },
    Emoji { glyph: "🔖", name: "bookmark", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🔗", name: "link symbol", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🔘", name: "radio button", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🔙", name: "back with leftwards arrow above", cat: "Symbols" },
    Emoji { glyph: "🔚", name: "end with leftwards arrow above", cat: "Symbols" },
    Emoji { glyph: "🔛", name: "on with exclamation mark with left right arrow above", cat: "Symbols" },
    Emoji { glyph: "🔜", name: "soon with rightwards arrow above", cat: "Symbols" },
    Emoji { glyph: "🔝", name: "top with upwards arrow above", cat: "Symbols" },
    Emoji { glyph: "🔞", name: "no one under eighteen symbol", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🔟", name: "keycap ten", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🔠", name: "input symbol for latin capital letters", cat: "Symbols" },
    Emoji { glyph: "🔡", name: "input symbol for latin small letters", cat: "Symbols" },
    Emoji { glyph: "🔢", name: "input symbol for numbers", cat: "Symbols" },
    Emoji { glyph: "🔣", name: "input symbol for symbols", cat: "Symbols" },
    Emoji { glyph: "🔤", name: "input symbol for latin letters", cat: "Symbols" },
    Emoji { glyph: "🔩", name: "nut and bolt", cat: "Symbols" },
    Emoji { glyph: "🔪", name: "hocho", cat: "Symbols" },
    Emoji { glyph: "🔫", name: "pistol", cat: "Symbols" },
    Emoji { glyph: "🔬", name: "microscope", cat: "Symbols" },
    Emoji { glyph: "🔭", name: "telescope", cat: "Symbols" },
    Emoji { glyph: "🔮", name: "crystal ball", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🔯", name: "six pointed star with middle dot", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🔰", name: "japanese symbol for beginner", cat: "Symbols" },
    Emoji { glyph: "🔱", name: "trident emblem", cat: "Symbols" },
    Emoji { glyph: "🔲", name: "black square button", cat: "Symbols" },
    Emoji { glyph: "🔳", name: "white square button", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🔶", name: "large orange diamond", cat: "Objects" },
    Emoji { glyph: "🔷", name: "large blue diamond", cat: "Objects" },
    Emoji { glyph: "🔸", name: "small orange diamond", cat: "Objects" },
    Emoji { glyph: "🔹", name: "small blue diamond", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🔺", name: "up pointing red triangle", cat: "Symbols" },
    Emoji { glyph: "🔻", name: "down pointing red triangle", cat: "Symbols" },
    Emoji { glyph: "🔼", name: "up pointing small red triangle", cat: "Symbols" },
    Emoji { glyph: "🔽", name: "down pointing small red triangle", cat: "Symbols" },
    Emoji { glyph: "🕉️", name: "om symbol", cat: "Symbols" },
    Emoji { glyph: "🕊️", name: "dove of peace", cat: "Symbols" },
    Emoji { glyph: "🕋", name: "kaaba", cat: "Symbols" },
    Emoji { glyph: "🕌", name: "mosque", cat: "Symbols" },
    Emoji { glyph: "🕍", name: "synagogue", cat: "Symbols" },
    Emoji { glyph: "🕎", name: "menorah with nine branches", cat: "Symbols" },

    // People
    Emoji { glyph: "🕐", name: "clock face one oclock", cat: "People" },
    Emoji { glyph: "🕑", name: "clock face two oclock", cat: "People" },
    Emoji { glyph: "🕒", name: "clock face three oclock", cat: "People" },
    Emoji { glyph: "🕓", name: "clock face four oclock", cat: "People" },
    Emoji { glyph: "🕔", name: "clock face five oclock", cat: "People" },
    Emoji { glyph: "🕕", name: "clock face six oclock", cat: "People" },
    Emoji { glyph: "🕖", name: "clock face seven oclock", cat: "People" },
    Emoji { glyph: "🕗", name: "clock face eight oclock", cat: "People" },
    Emoji { glyph: "🕘", name: "clock face nine oclock", cat: "People" },
    Emoji { glyph: "🕙", name: "clock face ten oclock", cat: "People" },
    Emoji { glyph: "🕚", name: "clock face eleven oclock", cat: "People" },
    Emoji { glyph: "🕛", name: "clock face twelve oclock", cat: "People" },
    Emoji { glyph: "🕜", name: "clock face one thirty", cat: "People" },
    Emoji { glyph: "🕝", name: "clock face two thirty", cat: "People" },
    Emoji { glyph: "🕞", name: "clock face three thirty", cat: "People" },
    Emoji { glyph: "🕟", name: "clock face four thirty", cat: "People" },
    Emoji { glyph: "🕠", name: "clock face five thirty", cat: "People" },
    Emoji { glyph: "🕡", name: "clock face six thirty", cat: "People" },
    Emoji { glyph: "🕢", name: "clock face seven thirty", cat: "People" },
    Emoji { glyph: "🕣", name: "clock face eight thirty", cat: "People" },
    Emoji { glyph: "🕤", name: "clock face nine thirty", cat: "People" },
    Emoji { glyph: "🕥", name: "clock face ten thirty", cat: "People" },
    Emoji { glyph: "🕦", name: "clock face eleven thirty", cat: "People" },
    Emoji { glyph: "🕧", name: "clock face twelve thirty", cat: "People" },
    Emoji { glyph: "🕰️", name: "mantelpiece clock", cat: "People" },

    // Symbols
    Emoji { glyph: "🕳️", name: "hole", cat: "Symbols" },

    // People
    Emoji { glyph: "🕴️", name: "man in business suit levitating", cat: "People" },

    // Symbols
    Emoji { glyph: "🕵️", name: "sleuth or spy", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🕶️", name: "dark sunglasses", cat: "Nature" },

    // Animals
    Emoji { glyph: "🕷️", name: "spider", cat: "Animals" },
    Emoji { glyph: "🕸️", name: "spider web", cat: "Animals" },

    // People
    Emoji { glyph: "🕺", name: "man dancing", cat: "People" },

    // Objects
    Emoji { glyph: "🖇️", name: "linked paperclips", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🖊️", name: "lower left ballpoint pen", cat: "Symbols" },
    Emoji { glyph: "🖋️", name: "lower left fountain pen", cat: "Symbols" },
    Emoji { glyph: "🖌️", name: "lower left paintbrush", cat: "Symbols" },
    Emoji { glyph: "🖍️", name: "lower left crayon", cat: "Symbols" },

    // People
    Emoji { glyph: "🖕", name: "reversed hand with middle finger extended", cat: "People" },
    Emoji { glyph: "🖖", name: "raised hand with part between middle and ring fingers", cat: "People" },

    // Activities
    Emoji { glyph: "🖤", name: "black heart", cat: "Activities" },

    // Symbols
    Emoji { glyph: "🖲️", name: "trackball", cat: "Symbols" },
    Emoji { glyph: "🖼️", name: "frame with picture", cat: "Symbols" },

    // Travel
    Emoji { glyph: "🗃️", name: "card file box", cat: "Travel" },

    // Objects
    Emoji { glyph: "🗄️", name: "file cabinet", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🗒️", name: "spiral note pad", cat: "Symbols" },
    Emoji { glyph: "🗓️", name: "spiral calendar pad", cat: "Symbols" },
    Emoji { glyph: "🗜️", name: "compression", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🗝️", name: "old key", cat: "Objects" },
    Emoji { glyph: "🗞️", name: "rolled up newspaper", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🗡️", name: "dagger knife", cat: "Symbols" },

    // People
    Emoji { glyph: "🗣️", name: "speaking head in silhouette", cat: "People" },

    // Objects
    Emoji { glyph: "🗨️", name: "left speech bubble", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🗯️", name: "right anger bubble", cat: "Symbols" },
    Emoji { glyph: "🗳️", name: "ballot box with ballot", cat: "Symbols" },
    Emoji { glyph: "🗺️", name: "world map", cat: "Symbols" },
    Emoji { glyph: "🗻", name: "mount fuji", cat: "Symbols" },
    Emoji { glyph: "🗾", name: "silhouette of japan", cat: "Symbols" },
    Emoji { glyph: "🗿", name: "moyai", cat: "Symbols" },

    // People
    Emoji { glyph: "😈", name: "smiling face with horns", cat: "People" },
    Emoji { glyph: "😌", name: "relieved face", cat: "People" },
    Emoji { glyph: "😒", name: "unamused face", cat: "People" },
    Emoji { glyph: "😓", name: "face with cold sweat", cat: "People" },
    Emoji { glyph: "😔", name: "pensive face", cat: "People" },
    Emoji { glyph: "😕", name: "confused face", cat: "People" },
    Emoji { glyph: "😖", name: "confounded face", cat: "People" },
    Emoji { glyph: "😗", name: "kissing face", cat: "People" },
    Emoji { glyph: "😙", name: "kissing face with smiling eyes", cat: "People" },
    Emoji { glyph: "😚", name: "kissing face with closed eyes", cat: "People" },
    Emoji { glyph: "😛", name: "face with stuck out tongue", cat: "People" },
    Emoji { glyph: "😜", name: "face with stuck out tongue and winking eye", cat: "People" },
    Emoji { glyph: "😝", name: "face with stuck out tongue and tightly closed eyes", cat: "People" },
    Emoji { glyph: "😞", name: "disappointed face", cat: "People" },
    Emoji { glyph: "😟", name: "worried face", cat: "People" },
    Emoji { glyph: "😠", name: "angry face", cat: "People" },
    Emoji { glyph: "😣", name: "persevering face", cat: "People" },
    Emoji { glyph: "😤", name: "face with look of triumph", cat: "People" },
    Emoji { glyph: "😥", name: "disappointed but relieved face", cat: "People" },
    Emoji { glyph: "😦", name: "frowning face with open mouth", cat: "People" },
    Emoji { glyph: "😧", name: "anguished face", cat: "People" },
    Emoji { glyph: "😨", name: "fearful face", cat: "People" },
    Emoji { glyph: "😩", name: "weary face", cat: "People" },
    Emoji { glyph: "😪", name: "sleepy face", cat: "People" },
    Emoji { glyph: "😫", name: "tired face", cat: "People" },
    Emoji { glyph: "😬", name: "grimacing face", cat: "People" },
    Emoji { glyph: "😮", name: "face with open mouth", cat: "People" },
    Emoji { glyph: "😯", name: "hushed face", cat: "People" },
    Emoji { glyph: "😰", name: "face with open mouth and cold sweat", cat: "People" },
    Emoji { glyph: "😲", name: "astonished face", cat: "People" },
    Emoji { glyph: "😳", name: "flushed face", cat: "People" },
    Emoji { glyph: "😵", name: "dizzy face", cat: "People" },
    Emoji { glyph: "😷", name: "face with medical mask", cat: "People" },
    Emoji { glyph: "😸", name: "grinning cat face with smiling eyes", cat: "People" },
    Emoji { glyph: "😹", name: "cat face with tears of joy", cat: "People" },
    Emoji { glyph: "😺", name: "smiling cat face with open mouth", cat: "People" },
    Emoji { glyph: "😻", name: "smiling cat face with heart shaped eyes", cat: "People" },
    Emoji { glyph: "😼", name: "cat face with wry smile", cat: "People" },
    Emoji { glyph: "😽", name: "kissing cat face with closed eyes", cat: "People" },
    Emoji { glyph: "😾", name: "pouting cat face", cat: "People" },
    Emoji { glyph: "😿", name: "crying cat face", cat: "People" },
    Emoji { glyph: "🙀", name: "weary cat face", cat: "People" },
    Emoji { glyph: "🙁", name: "slightly frowning face", cat: "People" },
    Emoji { glyph: "🙅", name: "face with no good gesture", cat: "People" },
    Emoji { glyph: "🙆", name: "face with ok gesture", cat: "People" },
    Emoji { glyph: "🙇", name: "person bowing deeply", cat: "People" },

    // Animals
    Emoji { glyph: "🙈", name: "see no evil monkey", cat: "Animals" },
    Emoji { glyph: "🙉", name: "hear no evil monkey", cat: "Animals" },
    Emoji { glyph: "🙊", name: "speak no evil monkey", cat: "Animals" },

    // People
    Emoji { glyph: "🙋", name: "happy person raising one hand", cat: "People" },
    Emoji { glyph: "🙍", name: "person frowning", cat: "People" },
    Emoji { glyph: "🙎", name: "person with pouting face", cat: "People" },

    // Travel
    Emoji { glyph: "🚃", name: "railway car", cat: "Travel" },

    // Nature
    Emoji { glyph: "🚄", name: "high speed train", cat: "Nature" },

    // People
    Emoji { glyph: "🚅", name: "high speed train with bullet nose", cat: "People" },

    // Symbols
    Emoji { glyph: "🚈", name: "light rail", cat: "Symbols" },
    Emoji { glyph: "🚊", name: "tram", cat: "Symbols" },

    // Travel
    Emoji { glyph: "🚋", name: "tram car", cat: "Travel" },
    Emoji { glyph: "🚍", name: "oncoming bus", cat: "Travel" },
    Emoji { glyph: "🚏", name: "bus stop", cat: "Travel" },
    Emoji { glyph: "🚐", name: "minibus", cat: "Travel" },
    Emoji { glyph: "🚔", name: "oncoming police car", cat: "Travel" },
    Emoji { glyph: "🚖", name: "oncoming taxi", cat: "Travel" },

    // Symbols
    Emoji { glyph: "🚘", name: "oncoming automobile", cat: "Symbols" },
    Emoji { glyph: "🚙", name: "recreational vehicle", cat: "Symbols" },

    // Activities
    Emoji { glyph: "🚛", name: "articulated lorry", cat: "Activities" },

    // Travel
    Emoji { glyph: "🚜", name: "tractor", cat: "Travel" },

    // Symbols
    Emoji { glyph: "🚝", name: "monorail", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🚞", name: "mountain railway", cat: "Nature" },

    // Travel
    Emoji { glyph: "🚟", name: "suspension railway", cat: "Travel" },

    // Nature
    Emoji { glyph: "🚠", name: "mountain cableway", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🚡", name: "aerial tramway", cat: "Symbols" },

    // Travel
    Emoji { glyph: "🚣", name: "rowboat", cat: "Travel" },
    Emoji { glyph: "🚤", name: "speedboat", cat: "Travel" },
    Emoji { glyph: "🚥", name: "horizontal traffic light", cat: "Travel" },
    Emoji { glyph: "🚦", name: "vertical traffic light", cat: "Travel" },
    Emoji { glyph: "🚨", name: "police cars revolving light", cat: "Travel" },

    // Symbols
    Emoji { glyph: "🚩", name: "triangular flag on post", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🚪", name: "door", cat: "Objects" },

    // People
    Emoji { glyph: "🚬", name: "smoking symbol", cat: "People" },
    Emoji { glyph: "🚭", name: "no smoking symbol", cat: "People" },

    // Symbols
    Emoji { glyph: "🚮", name: "put litter in its place symbol", cat: "Symbols" },
    Emoji { glyph: "🚯", name: "do not litter symbol", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🚰", name: "potable water symbol", cat: "Objects" },
    Emoji { glyph: "🚱", name: "non potable water symbol", cat: "Objects" },

    // Travel
    Emoji { glyph: "🚳", name: "no bicycles", cat: "Travel" },

    // Symbols
    Emoji { glyph: "🚴", name: "bicyclist", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🚵", name: "mountain bicyclist", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🚶", name: "pedestrian", cat: "Symbols" },
    Emoji { glyph: "🚷", name: "no pedestrians", cat: "Symbols" },

    // People
    Emoji { glyph: "🚸", name: "children crossing", cat: "People" },

    // Symbols
    Emoji { glyph: "🚹", name: "mens symbol", cat: "Symbols" },
    Emoji { glyph: "🚺", name: "womens symbol", cat: "Symbols" },
    Emoji { glyph: "🚻", name: "restroom", cat: "Symbols" },

    // People
    Emoji { glyph: "🚼", name: "baby symbol", cat: "People" },

    // Symbols
    Emoji { glyph: "🚽", name: "toilet", cat: "Symbols" },
    Emoji { glyph: "🚾", name: "water closet", cat: "Symbols" },
    Emoji { glyph: "🚿", name: "shower", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🛀", name: "bath", cat: "Animals" },
    Emoji { glyph: "🛁", name: "bathtub", cat: "Animals" },

    // Activities
    Emoji { glyph: "🛂", name: "passport control", cat: "Activities" },

    // Symbols
    Emoji { glyph: "🛃", name: "customs", cat: "Symbols" },
    Emoji { glyph: "🛄", name: "baggage claim", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🛅", name: "left luggage", cat: "Objects" },
    Emoji { glyph: "🛋️", name: "couch and lamp", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🛌", name: "sleeping accommodation", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🛍️", name: "shopping bags", cat: "Objects" },
    Emoji { glyph: "🛎️", name: "bellhop bell", cat: "Objects" },
    Emoji { glyph: "🛏️", name: "bed", cat: "Objects" },

    // Travel
    Emoji { glyph: "🛐", name: "place of worship", cat: "Travel" },

    // Objects
    Emoji { glyph: "🛑", name: "octagonal sign", cat: "Objects" },
    Emoji { glyph: "🛒", name: "shopping trolley", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🛕", name: "hindu temple", cat: "Symbols" },
    Emoji { glyph: "🛖", name: "hut", cat: "Symbols" },
    Emoji { glyph: "🛗", name: "elevator", cat: "Symbols" },
    Emoji { glyph: "🛜", name: "wireless", cat: "Symbols" },
    Emoji { glyph: "🛝", name: "playground slide", cat: "Symbols" },
    Emoji { glyph: "🛞", name: "wheel", cat: "Symbols" },
    Emoji { glyph: "🛟", name: "ring buoy", cat: "Symbols" },
    Emoji { glyph: "🛡️", name: "shield", cat: "Symbols" },

    // Activities
    Emoji { glyph: "🛢️", name: "oil drum", cat: "Activities" },

    // Symbols
    Emoji { glyph: "🛣️", name: "motorway", cat: "Symbols" },

    // Travel
    Emoji { glyph: "🛤️", name: "railway track", cat: "Travel" },
    Emoji { glyph: "🛥️", name: "motor boat", cat: "Travel" },
    Emoji { glyph: "🛩️", name: "small airplane", cat: "Travel" },

    // Activities
    Emoji { glyph: "🛫", name: "airplane departure", cat: "Activities" },

    // Travel
    Emoji { glyph: "🛬", name: "airplane arriving", cat: "Travel" },

    // Symbols
    Emoji { glyph: "🛰️", name: "satellite", cat: "Symbols" },

    // Travel
    Emoji { glyph: "🛳️", name: "passenger ship", cat: "Travel" },
    Emoji { glyph: "🛴", name: "scooter", cat: "Travel" },
    Emoji { glyph: "🛶", name: "canoe", cat: "Travel" },

    // Symbols
    Emoji { glyph: "🛷", name: "sled", cat: "Symbols" },
    Emoji { glyph: "🛹", name: "skateboard", cat: "Symbols" },
    Emoji { glyph: "🛺", name: "auto rickshaw", cat: "Symbols" },

    // Travel
    Emoji { glyph: "🛻", name: "pickup truck", cat: "Travel" },

    // Symbols
    Emoji { glyph: "🛼", name: "roller skate", cat: "Symbols" },
    Emoji { glyph: "🟤", name: "large brown circle", cat: "Symbols" },
    Emoji { glyph: "🟫", name: "large brown square", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🟰", name: "heavy equals sign", cat: "Objects" },

    // Activities
    Emoji { glyph: "🤍", name: "white heart", cat: "Activities" },
    Emoji { glyph: "🤎", name: "brown heart", cat: "Activities" },

    // People
    Emoji { glyph: "🤐", name: "zipper mouth face", cat: "People" },
    Emoji { glyph: "🤑", name: "money mouth face", cat: "People" },
    Emoji { glyph: "🤒", name: "face with thermometer", cat: "People" },
    Emoji { glyph: "🤕", name: "face with head bandage", cat: "People" },
    Emoji { glyph: "🤖", name: "robot face", cat: "People" },
    Emoji { glyph: "🤛", name: "left facing fist", cat: "People" },
    Emoji { glyph: "🤜", name: "right facing fist", cat: "People" },
    Emoji { glyph: "🤠", name: "face with cowboy hat", cat: "People" },
    Emoji { glyph: "🤡", name: "clown face", cat: "People" },
    Emoji { glyph: "🤢", name: "nauseated face", cat: "People" },
    Emoji { glyph: "🤤", name: "drooling face", cat: "People" },
    Emoji { glyph: "🤥", name: "lying face", cat: "People" },
    Emoji { glyph: "🤦", name: "face palm", cat: "People" },
    Emoji { glyph: "🤧", name: "sneezing face", cat: "People" },
    Emoji { glyph: "🤩", name: "grinning face with star eyes", cat: "People" },
    Emoji { glyph: "🤪", name: "grinning face with one large and one small eye", cat: "People" },
    Emoji { glyph: "🤭", name: "smiling face with smiling eyes and hand covering mouth", cat: "People" },
    Emoji { glyph: "🤮", name: "face with open mouth vomiting", cat: "People" },
    Emoji { glyph: "🤰", name: "pregnant woman", cat: "People" },

    // Symbols
    Emoji { glyph: "🤱", name: "breast feeding", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🤲", name: "palms up together", cat: "Nature" },

    // People
    Emoji { glyph: "🤳", name: "selfie", cat: "People" },
    Emoji { glyph: "🤴", name: "prince", cat: "People" },
    Emoji { glyph: "🤵", name: "man in tuxedo", cat: "People" },

    // Symbols
    Emoji { glyph: "🤶", name: "mother christmas", cat: "Symbols" },
    Emoji { glyph: "🤷", name: "shrug", cat: "Symbols" },

    // People
    Emoji { glyph: "🤸", name: "person doing cartwheel", cat: "People" },

    // Symbols
    Emoji { glyph: "🤹", name: "juggling", cat: "Symbols" },
    Emoji { glyph: "🤺", name: "fencer", cat: "Symbols" },
    Emoji { glyph: "🤼", name: "wrestlers", cat: "Symbols" },
    Emoji { glyph: "🤽", name: "water polo", cat: "Symbols" },

    // People
    Emoji { glyph: "🤾", name: "handball", cat: "People" },

    // Symbols
    Emoji { glyph: "🤿", name: "diving mask", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🥀", name: "wilted flower", cat: "Nature" },

    // People
    Emoji { glyph: "🥂", name: "clinking glasses", cat: "People" },

    // Symbols
    Emoji { glyph: "🥃", name: "tumbler glass", cat: "Symbols" },
    Emoji { glyph: "🥄", name: "spoon", cat: "Symbols" },
    Emoji { glyph: "🥅", name: "goal net", cat: "Symbols" },
    Emoji { glyph: "🥌", name: "curling stone", cat: "Symbols" },
    Emoji { glyph: "🥍", name: "lacrosse stick and ball", cat: "Symbols" },

    // Food
    Emoji { glyph: "🥐", name: "croissant", cat: "Food" },
    Emoji { glyph: "🥒", name: "cucumber", cat: "Food" },
    Emoji { glyph: "🥓", name: "bacon", cat: "Food" },
    Emoji { glyph: "🥔", name: "potato", cat: "Food" },
    Emoji { glyph: "🥖", name: "baguette bread", cat: "Food" },
    Emoji { glyph: "🥗", name: "green salad", cat: "Food" },
    Emoji { glyph: "🥘", name: "shallow pan of food", cat: "Food" },
    Emoji { glyph: "🥙", name: "stuffed flatbread", cat: "Food" },

    // Symbols
    Emoji { glyph: "🥚", name: "egg", cat: "Symbols" },
    Emoji { glyph: "🥛", name: "glass of milk", cat: "Symbols" },
    Emoji { glyph: "🥜", name: "peanuts", cat: "Symbols" },
    Emoji { glyph: "🥝", name: "kiwifruit", cat: "Symbols" },

    // Food
    Emoji { glyph: "🥞", name: "pancakes", cat: "Food" },

    // Symbols
    Emoji { glyph: "🥟", name: "dumpling", cat: "Symbols" },

    // People
    Emoji { glyph: "🥠", name: "fortune cookie", cat: "People" },

    // Symbols
    Emoji { glyph: "🥡", name: "takeout box", cat: "Symbols" },
    Emoji { glyph: "🥢", name: "chopsticks", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🥣", name: "bowl with spoon", cat: "Animals" },

    // Food
    Emoji { glyph: "🥦", name: "broccoli", cat: "Food" },

    // Symbols
    Emoji { glyph: "🥧", name: "pie", cat: "Symbols" },
    Emoji { glyph: "🥨", name: "pretzel", cat: "Symbols" },

    // Food
    Emoji { glyph: "🥩", name: "cut of meat", cat: "Food" },

    // Symbols
    Emoji { glyph: "🥪", name: "sandwich", cat: "Symbols" },

    // Food
    Emoji { glyph: "🥫", name: "canned food", cat: "Food" },

    // Nature
    Emoji { glyph: "🥬", name: "leafy green", cat: "Nature" },
    Emoji { glyph: "🥮", name: "moon cake", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🥯", name: "bagel", cat: "Symbols" },

    // People
    Emoji { glyph: "🥲", name: "smiling face with tear", cat: "People" },
    Emoji { glyph: "🥴", name: "face with uneven eyes and wavy mouth", cat: "People" },
    Emoji { glyph: "🥵", name: "overheated face", cat: "People" },
    Emoji { glyph: "🥶", name: "freezing face", cat: "People" },

    // Symbols
    Emoji { glyph: "🥷", name: "ninja", cat: "Symbols" },

    // People
    Emoji { glyph: "🥸", name: "disguised face", cat: "People" },
    Emoji { glyph: "🥹", name: "face holding back tears", cat: "People" },

    // Symbols
    Emoji { glyph: "🥻", name: "sari", cat: "Symbols" },
    Emoji { glyph: "🥼", name: "lab coat", cat: "Symbols" },
    Emoji { glyph: "🥽", name: "goggles", cat: "Symbols" },

    // People
    Emoji { glyph: "🥾", name: "hiking boot", cat: "People" },

    // Objects
    Emoji { glyph: "🥿", name: "flat shoe", cat: "Objects" },

    // Animals
    Emoji { glyph: "🦂", name: "scorpion", cat: "Animals" },
    Emoji { glyph: "🦃", name: "turkey", cat: "Animals" },
    Emoji { glyph: "🦅", name: "eagle", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🦌", name: "deer", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🦎", name: "lizard", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🦏", name: "rhinoceros", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🦐", name: "shrimp", cat: "Animals" },
    Emoji { glyph: "🦑", name: "squid", cat: "Animals" },
    Emoji { glyph: "🦔", name: "hedgehog", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🦕", name: "sauropod", cat: "Symbols" },

    // Activities
    Emoji { glyph: "🦗", name: "cricket", cat: "Activities" },

    // Animals
    Emoji { glyph: "🦘", name: "kangaroo", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🦙", name: "llama", cat: "Symbols" },
    Emoji { glyph: "🦚", name: "peacock", cat: "Symbols" },
    Emoji { glyph: "🦛", name: "hippopotamus", cat: "Symbols" },
    Emoji { glyph: "🦜", name: "parrot", cat: "Symbols" },
    Emoji { glyph: "🦝", name: "raccoon", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🦞", name: "lobster", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🦟", name: "mosquito", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🦠", name: "microbe", cat: "Animals" },
    Emoji { glyph: "🦡", name: "badger", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🦢", name: "swan", cat: "Symbols" },
    Emoji { glyph: "🦣", name: "mammoth", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🦤", name: "dodo", cat: "Animals" },
    Emoji { glyph: "🦥", name: "sloth", cat: "Animals" },
    Emoji { glyph: "🦦", name: "otter", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🦧", name: "orangutan", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🦨", name: "skunk", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🦩", name: "flamingo", cat: "Symbols" },
    Emoji { glyph: "🦪", name: "oyster", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🦫", name: "beaver", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🦬", name: "bison", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🦮", name: "guide dog", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🦯", name: "probing cane", cat: "Symbols" },
    Emoji { glyph: "🦰", name: "emoji component red hair", cat: "Symbols" },
    Emoji { glyph: "🦱", name: "emoji component curly hair", cat: "Symbols" },
    Emoji { glyph: "🦲", name: "emoji component bald", cat: "Symbols" },
    Emoji { glyph: "🦳", name: "emoji component white hair", cat: "Symbols" },
    Emoji { glyph: "🦴", name: "bone", cat: "Symbols" },

    // People
    Emoji { glyph: "🦵", name: "leg", cat: "People" },
    Emoji { glyph: "🦶", name: "foot", cat: "People" },

    // Symbols
    Emoji { glyph: "🦷", name: "tooth", cat: "Symbols" },

    // People
    Emoji { glyph: "🦸", name: "superhero", cat: "People" },
    Emoji { glyph: "🦹", name: "supervillain", cat: "People" },

    // Symbols
    Emoji { glyph: "🦺", name: "safety vest", cat: "Symbols" },

    // People
    Emoji { glyph: "🦻", name: "ear with hearing aid", cat: "People" },
    Emoji { glyph: "🦼", name: "motorized wheelchair", cat: "People" },
    Emoji { glyph: "🦽", name: "manual wheelchair", cat: "People" },
    Emoji { glyph: "🦾", name: "mechanical arm", cat: "People" },
    Emoji { glyph: "🦿", name: "mechanical leg", cat: "People" },

    // Food
    Emoji { glyph: "🧀", name: "cheese wedge", cat: "Food" },
    Emoji { glyph: "🧁", name: "cupcake", cat: "Food" },
    Emoji { glyph: "🧄", name: "garlic", cat: "Food" },
    Emoji { glyph: "🧅", name: "onion", cat: "Food" },

    // Symbols
    Emoji { glyph: "🧆", name: "falafel", cat: "Symbols" },

    // Food
    Emoji { glyph: "🧇", name: "waffle", cat: "Food" },

    // Symbols
    Emoji { glyph: "🧈", name: "butter", cat: "Symbols" },

    // Food
    Emoji { glyph: "🧉", name: "mate drink", cat: "Food" },

    // Symbols
    Emoji { glyph: "🧊", name: "ice cube", cat: "Symbols" },
    Emoji { glyph: "🧌", name: "troll", cat: "Symbols" },

    // People
    Emoji { glyph: "🧍", name: "standing person", cat: "People" },
    Emoji { glyph: "🧎", name: "kneeling person", cat: "People" },
    Emoji { glyph: "🧏", name: "deaf person", cat: "People" },
    Emoji { glyph: "🧐", name: "face with monocle", cat: "People" },

    // Symbols
    Emoji { glyph: "🧑", name: "adult", cat: "Symbols" },

    // People
    Emoji { glyph: "🧒", name: "child", cat: "People" },

    // Symbols
    Emoji { glyph: "🧓", name: "older adult", cat: "Symbols" },

    // People
    Emoji { glyph: "🧔", name: "bearded person", cat: "People" },
    Emoji { glyph: "🧕", name: "person with headscarf", cat: "People" },
    Emoji { glyph: "🧖", name: "person in steamy room", cat: "People" },
    Emoji { glyph: "🧗", name: "person climbing", cat: "People" },
    Emoji { glyph: "🧘", name: "person in lotus position", cat: "People" },
    Emoji { glyph: "🧙", name: "mage", cat: "People" },
    Emoji { glyph: "🧚", name: "fairy", cat: "People" },
    Emoji { glyph: "🧛", name: "vampire", cat: "People" },
    Emoji { glyph: "🧜", name: "merperson", cat: "People" },
    Emoji { glyph: "🧝", name: "elf", cat: "People" },
    Emoji { glyph: "🧞", name: "genie", cat: "People" },
    Emoji { glyph: "🧟", name: "zombie", cat: "People" },

    // Nature
    Emoji { glyph: "🧠", name: "brain", cat: "Nature" },

    // Activities
    Emoji { glyph: "🧡", name: "orange heart", cat: "Activities" },

    // Symbols
    Emoji { glyph: "🧢", name: "billed cap", cat: "Symbols" },

    // Travel
    Emoji { glyph: "🧣", name: "scarf", cat: "Travel" },

    // Symbols
    Emoji { glyph: "🧤", name: "gloves", cat: "Symbols" },
    Emoji { glyph: "🧥", name: "coat", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🧦", name: "socks", cat: "Objects" },
    Emoji { glyph: "🧧", name: "red gift envelope", cat: "Objects" },

    // Nature
    Emoji { glyph: "🧨", name: "firecracker", cat: "Nature" },

    // Activities
    Emoji { glyph: "🧩", name: "jigsaw puzzle piece", cat: "Activities" },

    // Symbols
    Emoji { glyph: "🧫", name: "petri dish", cat: "Symbols" },
    Emoji { glyph: "🧬", name: "dna double helix", cat: "Symbols" },
    Emoji { glyph: "🧭", name: "compass", cat: "Symbols" },
    Emoji { glyph: "🧮", name: "abacus", cat: "Symbols" },
    Emoji { glyph: "🧱", name: "brick", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🧳", name: "luggage", cat: "Objects" },

    // Food
    Emoji { glyph: "🧴", name: "lotion bottle", cat: "Food" },

    // Symbols
    Emoji { glyph: "🧵", name: "spool of thread", cat: "Symbols" },
    Emoji { glyph: "🧶", name: "ball of yarn", cat: "Symbols" },
    Emoji { glyph: "🧷", name: "safety pin", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🧹", name: "broom", cat: "Objects" },
    Emoji { glyph: "🧺", name: "basket", cat: "Objects" },
    Emoji { glyph: "🧻", name: "roll of paper", cat: "Objects" },
    Emoji { glyph: "🧼", name: "bar of soap", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🧽", name: "sponge", cat: "Symbols" },
    Emoji { glyph: "🧾", name: "receipt", cat: "Symbols" },
    Emoji { glyph: "🧿", name: "nazar amulet", cat: "Symbols" },

    // Activities
    Emoji { glyph: "🩰", name: "ballet shoes", cat: "Activities" },
    Emoji { glyph: "🩱", name: "one piece swimsuit", cat: "Activities" },

    // Symbols
    Emoji { glyph: "🩲", name: "briefs", cat: "Symbols" },
    Emoji { glyph: "🩳", name: "shorts", cat: "Symbols" },
    Emoji { glyph: "🩴", name: "thong sandal", cat: "Symbols" },

    // Activities
    Emoji { glyph: "🩵", name: "light blue heart", cat: "Activities" },
    Emoji { glyph: "🩶", name: "grey heart", cat: "Activities" },
    Emoji { glyph: "🩷", name: "pink heart", cat: "Activities" },

    // Symbols
    Emoji { glyph: "🩸", name: "drop of blood", cat: "Symbols" },
    Emoji { glyph: "🩹", name: "adhesive bandage", cat: "Symbols" },
    Emoji { glyph: "🩺", name: "stethoscope", cat: "Symbols" },
    Emoji { glyph: "🩻", name: "x ray", cat: "Symbols" },
    Emoji { glyph: "🩼", name: "crutch", cat: "Symbols" },
    Emoji { glyph: "🪀", name: "yo yo", cat: "Symbols" },
    Emoji { glyph: "🪁", name: "kite", cat: "Symbols" },
    Emoji { glyph: "🪂", name: "parachute", cat: "Symbols" },
    Emoji { glyph: "🪃", name: "boomerang", cat: "Symbols" },
    Emoji { glyph: "🪅", name: "pinata", cat: "Symbols" },
    Emoji { glyph: "🪆", name: "nesting dolls", cat: "Symbols" },
    Emoji { glyph: "🪇", name: "maracas", cat: "Symbols" },
    Emoji { glyph: "🪈", name: "flute", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🪐", name: "ringed planet", cat: "Nature" },

    // Objects
    Emoji { glyph: "🪑", name: "chair", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🪒", name: "razor", cat: "Symbols" },
    Emoji { glyph: "🪓", name: "axe", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🪔", name: "diya lamp", cat: "Objects" },

    // Symbols
    Emoji { glyph: "🪕", name: "banjo", cat: "Symbols" },
    Emoji { glyph: "🪖", name: "military helmet", cat: "Symbols" },
    Emoji { glyph: "🪗", name: "accordion", cat: "Symbols" },

    // Activities
    Emoji { glyph: "🪘", name: "long drum", cat: "Activities" },

    // Symbols
    Emoji { glyph: "🪙", name: "coin", cat: "Symbols" },

    // Travel
    Emoji { glyph: "🪚", name: "carpentry saw", cat: "Travel" },

    // Symbols
    Emoji { glyph: "🪛", name: "screwdriver", cat: "Symbols" },
    Emoji { glyph: "🪜", name: "ladder", cat: "Symbols" },
    Emoji { glyph: "🪝", name: "hook", cat: "Symbols" },
    Emoji { glyph: "🪞", name: "mirror", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🪟", name: "window", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🪠", name: "plunger", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🪡", name: "sewing needle", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🪢", name: "knot", cat: "Symbols" },
    Emoji { glyph: "🪣", name: "bucket", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🪤", name: "mouse trap", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🪥", name: "toothbrush", cat: "Symbols" },
    Emoji { glyph: "🪦", name: "headstone", cat: "Symbols" },

    // Travel
    Emoji { glyph: "🪧", name: "placard", cat: "Travel" },

    // Symbols
    Emoji { glyph: "🪨", name: "rock", cat: "Symbols" },
    Emoji { glyph: "🪩", name: "mirror ball", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🪪", name: "identification card", cat: "Animals" },
    Emoji { glyph: "🪫", name: "low battery", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🪬", name: "hamsa", cat: "Symbols" },

    // People
    Emoji { glyph: "🪭", name: "folding hand fan", cat: "People" },

    // Symbols
    Emoji { glyph: "🪮", name: "hair pick", cat: "Symbols" },

    // People
    Emoji { glyph: "🪯", name: "khanda", cat: "People" },

    // Symbols
    Emoji { glyph: "🪰", name: "fly", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🪱", name: "worm", cat: "Animals" },
    Emoji { glyph: "🪲", name: "beetle", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🪳", name: "cockroach", cat: "Symbols" },

    // Nature
    Emoji { glyph: "🪴", name: "potted plant", cat: "Nature" },

    // Symbols
    Emoji { glyph: "🪵", name: "wood", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🪶", name: "feather", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🪷", name: "lotus", cat: "Symbols" },
    Emoji { glyph: "🪸", name: "coral", cat: "Symbols" },
    Emoji { glyph: "🪹", name: "empty nest", cat: "Symbols" },
    Emoji { glyph: "🪺", name: "nest with eggs", cat: "Symbols" },
    Emoji { glyph: "🪻", name: "hyacinth", cat: "Symbols" },

    // Animals
    Emoji { glyph: "🪼", name: "jellyfish", cat: "Animals" },
    Emoji { glyph: "🪽", name: "wing", cat: "Animals" },

    // Symbols
    Emoji { glyph: "🪿", name: "goose", cat: "Symbols" },

    // Activities
    Emoji { glyph: "🫀", name: "anatomical heart", cat: "Activities" },

    // Symbols
    Emoji { glyph: "🫁", name: "lungs", cat: "Symbols" },

    // People
    Emoji { glyph: "🫂", name: "people hugging", cat: "People" },
    Emoji { glyph: "🫃", name: "pregnant man", cat: "People" },
    Emoji { glyph: "🫄", name: "pregnant person", cat: "People" },
    Emoji { glyph: "🫅", name: "person with crown", cat: "People" },

    // Symbols
    Emoji { glyph: "🫎", name: "moose", cat: "Symbols" },

    // Objects
    Emoji { glyph: "🫏", name: "donkey", cat: "Objects" },

    // Food
    Emoji { glyph: "🫑", name: "bell pepper", cat: "Food" },

    // Symbols
    Emoji { glyph: "🫒", name: "olive", cat: "Symbols" },

    // Food
    Emoji { glyph: "🫓", name: "flatbread", cat: "Food" },

    // Symbols
    Emoji { glyph: "🫔", name: "tamale", cat: "Symbols" },
    Emoji { glyph: "🫕", name: "fondue", cat: "Symbols" },

    // Food
    Emoji { glyph: "🫖", name: "teapot", cat: "Food" },

    // Symbols
    Emoji { glyph: "🫗", name: "pouring liquid", cat: "Symbols" },
    Emoji { glyph: "🫘", name: "beans", cat: "Symbols" },
    Emoji { glyph: "🫙", name: "jar", cat: "Symbols" },
    Emoji { glyph: "🫚", name: "ginger root", cat: "Symbols" },
    Emoji { glyph: "🫛", name: "pea pod", cat: "Symbols" },

    // People
    Emoji { glyph: "🫢", name: "face with open eyes and hand over mouth", cat: "People" },
    Emoji { glyph: "🫣", name: "face with peeking eye", cat: "People" },
    Emoji { glyph: "🫤", name: "face with diagonal mouth", cat: "People" },
    Emoji { glyph: "🫥", name: "dotted line face", cat: "People" },

    // Symbols
    Emoji { glyph: "🫦", name: "biting lip", cat: "Symbols" },
    Emoji { glyph: "🫧", name: "bubbles", cat: "Symbols" },

    // People
    Emoji { glyph: "🫨", name: "shaking face", cat: "People" },
    Emoji { glyph: "🫰", name: "hand with index finger and thumb crossed", cat: "People" },
    Emoji { glyph: "🫱", name: "rightwards hand", cat: "People" },
    Emoji { glyph: "🫲", name: "leftwards hand", cat: "People" },
    Emoji { glyph: "🫳", name: "palm down hand", cat: "People" },
    Emoji { glyph: "🫴", name: "palm up hand", cat: "People" },

    // Symbols
    Emoji { glyph: "🫵", name: "index pointing at the viewer", cat: "Symbols" },

    // People
    Emoji { glyph: "🫷", name: "leftwards pushing hand", cat: "People" },
    Emoji { glyph: "🫸", name: "rightwards pushing hand", cat: "People" },

    // Flags
    Emoji { glyph: "🇦🇼", name: "flag aruba", cat: "Flags" },
    Emoji { glyph: "🇦🇫", name: "flag afghanistan", cat: "Flags" },
    Emoji { glyph: "🇦🇴", name: "flag angola", cat: "Flags" },
    Emoji { glyph: "🇦🇮", name: "flag anguilla", cat: "Flags" },
    Emoji { glyph: "🇦🇽", name: "flag land islands", cat: "Flags" },
    Emoji { glyph: "🇦🇱", name: "flag albania", cat: "Flags" },
    Emoji { glyph: "🇦🇩", name: "flag andorra", cat: "Flags" },
    Emoji { glyph: "🇦🇪", name: "flag united arab emirates", cat: "Flags" },
    Emoji { glyph: "🇦🇷", name: "flag argentina", cat: "Flags" },
    Emoji { glyph: "🇦🇲", name: "flag armenia", cat: "Flags" },
    Emoji { glyph: "🇦🇸", name: "flag american samoa", cat: "Flags" },
    Emoji { glyph: "🇦🇶", name: "flag antarctica", cat: "Flags" },
    Emoji { glyph: "🇹🇫", name: "flag french southern territories", cat: "Flags" },
    Emoji { glyph: "🇦🇬", name: "flag antigua and barbuda", cat: "Flags" },
    Emoji { glyph: "🇦🇺", name: "flag australia", cat: "Flags" },
    Emoji { glyph: "🇦🇹", name: "flag austria", cat: "Flags" },
    Emoji { glyph: "🇦🇿", name: "flag azerbaijan", cat: "Flags" },
    Emoji { glyph: "🇧🇮", name: "flag burundi", cat: "Flags" },
    Emoji { glyph: "🇧🇪", name: "flag belgium", cat: "Flags" },
    Emoji { glyph: "🇧🇯", name: "flag benin", cat: "Flags" },
    Emoji { glyph: "🇧🇶", name: "flag bonaire sint eustatius and saba", cat: "Flags" },
    Emoji { glyph: "🇧🇫", name: "flag burkina faso", cat: "Flags" },
    Emoji { glyph: "🇧🇩", name: "flag bangladesh", cat: "Flags" },
    Emoji { glyph: "🇧🇬", name: "flag bulgaria", cat: "Flags" },
    Emoji { glyph: "🇧🇭", name: "flag bahrain", cat: "Flags" },
    Emoji { glyph: "🇧🇸", name: "flag bahamas", cat: "Flags" },
    Emoji { glyph: "🇧🇦", name: "flag bosnia and herzegovina", cat: "Flags" },
    Emoji { glyph: "🇧🇱", name: "flag saint barth lemy", cat: "Flags" },
    Emoji { glyph: "🇧🇾", name: "flag belarus", cat: "Flags" },
    Emoji { glyph: "🇧🇿", name: "flag belize", cat: "Flags" },
    Emoji { glyph: "🇧🇲", name: "flag bermuda", cat: "Flags" },
    Emoji { glyph: "🇧🇴", name: "flag bolivia plurinational state of", cat: "Flags" },
    Emoji { glyph: "🇧🇷", name: "flag brazil", cat: "Flags" },
    Emoji { glyph: "🇧🇧", name: "flag barbados", cat: "Flags" },
    Emoji { glyph: "🇧🇳", name: "flag brunei darussalam", cat: "Flags" },
    Emoji { glyph: "🇧🇹", name: "flag bhutan", cat: "Flags" },
    Emoji { glyph: "🇧🇻", name: "flag bouvet island", cat: "Flags" },
    Emoji { glyph: "🇧🇼", name: "flag botswana", cat: "Flags" },
    Emoji { glyph: "🇨🇫", name: "flag central african republic", cat: "Flags" },
    Emoji { glyph: "🇨🇦", name: "flag canada", cat: "Flags" },
    Emoji { glyph: "🇨🇨", name: "flag cocos keeling islands", cat: "Flags" },
    Emoji { glyph: "🇨🇭", name: "flag switzerland", cat: "Flags" },
    Emoji { glyph: "🇨🇱", name: "flag chile", cat: "Flags" },
    Emoji { glyph: "🇨🇳", name: "flag china", cat: "Flags" },
    Emoji { glyph: "🇨🇮", name: "flag c te d ivoire", cat: "Flags" },
    Emoji { glyph: "🇨🇲", name: "flag cameroon", cat: "Flags" },
    Emoji { glyph: "🇨🇩", name: "flag congo the democratic republic of the", cat: "Flags" },
    Emoji { glyph: "🇨🇬", name: "flag congo", cat: "Flags" },
    Emoji { glyph: "🇨🇰", name: "flag cook islands", cat: "Flags" },
    Emoji { glyph: "🇨🇴", name: "flag colombia", cat: "Flags" },
    Emoji { glyph: "🇰🇲", name: "flag comoros", cat: "Flags" },
    Emoji { glyph: "🇨🇻", name: "flag cabo verde", cat: "Flags" },
    Emoji { glyph: "🇨🇷", name: "flag costa rica", cat: "Flags" },
    Emoji { glyph: "🇨🇺", name: "flag cuba", cat: "Flags" },
    Emoji { glyph: "🇨🇼", name: "flag cura ao", cat: "Flags" },
    Emoji { glyph: "🇨🇽", name: "flag christmas island", cat: "Flags" },
    Emoji { glyph: "🇰🇾", name: "flag cayman islands", cat: "Flags" },
    Emoji { glyph: "🇨🇾", name: "flag cyprus", cat: "Flags" },
    Emoji { glyph: "🇨🇿", name: "flag czechia", cat: "Flags" },
    Emoji { glyph: "🇩🇪", name: "flag germany", cat: "Flags" },
    Emoji { glyph: "🇩🇯", name: "flag djibouti", cat: "Flags" },
    Emoji { glyph: "🇩🇲", name: "flag dominica", cat: "Flags" },
    Emoji { glyph: "🇩🇰", name: "flag denmark", cat: "Flags" },
    Emoji { glyph: "🇩🇴", name: "flag dominican republic", cat: "Flags" },
    Emoji { glyph: "🇩🇿", name: "flag algeria", cat: "Flags" },
    Emoji { glyph: "🇪🇨", name: "flag ecuador", cat: "Flags" },
    Emoji { glyph: "🇪🇬", name: "flag egypt", cat: "Flags" },
    Emoji { glyph: "🇪🇷", name: "flag eritrea", cat: "Flags" },
    Emoji { glyph: "🇪🇭", name: "flag western sahara", cat: "Flags" },
    Emoji { glyph: "🇪🇸", name: "flag spain", cat: "Flags" },
    Emoji { glyph: "🇪🇪", name: "flag estonia", cat: "Flags" },
    Emoji { glyph: "🇪🇹", name: "flag ethiopia", cat: "Flags" },
    Emoji { glyph: "🇫🇮", name: "flag finland", cat: "Flags" },
    Emoji { glyph: "🇫🇯", name: "flag fiji", cat: "Flags" },
    Emoji { glyph: "🇫🇰", name: "flag falkland islands malvinas", cat: "Flags" },
    Emoji { glyph: "🇫🇷", name: "flag france", cat: "Flags" },
    Emoji { glyph: "🇫🇴", name: "flag faroe islands", cat: "Flags" },
    Emoji { glyph: "🇫🇲", name: "flag micronesia federated states of", cat: "Flags" },
    Emoji { glyph: "🇬🇦", name: "flag gabon", cat: "Flags" },
    Emoji { glyph: "🇬🇧", name: "flag united kingdom", cat: "Flags" },
    Emoji { glyph: "🇬🇪", name: "flag georgia", cat: "Flags" },
    Emoji { glyph: "🇬🇬", name: "flag guernsey", cat: "Flags" },
    Emoji { glyph: "🇬🇭", name: "flag ghana", cat: "Flags" },
    Emoji { glyph: "🇬🇮", name: "flag gibraltar", cat: "Flags" },
    Emoji { glyph: "🇬🇳", name: "flag guinea", cat: "Flags" },
    Emoji { glyph: "🇬🇵", name: "flag guadeloupe", cat: "Flags" },
    Emoji { glyph: "🇬🇲", name: "flag gambia", cat: "Flags" },
    Emoji { glyph: "🇬🇼", name: "flag guinea bissau", cat: "Flags" },
    Emoji { glyph: "🇬🇶", name: "flag equatorial guinea", cat: "Flags" },
    Emoji { glyph: "🇬🇷", name: "flag greece", cat: "Flags" },
    Emoji { glyph: "🇬🇩", name: "flag grenada", cat: "Flags" },
    Emoji { glyph: "🇬🇱", name: "flag greenland", cat: "Flags" },
    Emoji { glyph: "🇬🇹", name: "flag guatemala", cat: "Flags" },
    Emoji { glyph: "🇬🇫", name: "flag french guiana", cat: "Flags" },
    Emoji { glyph: "🇬🇺", name: "flag guam", cat: "Flags" },
    Emoji { glyph: "🇬🇾", name: "flag guyana", cat: "Flags" },
    Emoji { glyph: "🇭🇰", name: "flag hong kong", cat: "Flags" },
    Emoji { glyph: "🇭🇲", name: "flag heard island and mcdonald islands", cat: "Flags" },
    Emoji { glyph: "🇭🇳", name: "flag honduras", cat: "Flags" },
    Emoji { glyph: "🇭🇷", name: "flag croatia", cat: "Flags" },
    Emoji { glyph: "🇭🇹", name: "flag haiti", cat: "Flags" },
    Emoji { glyph: "🇭🇺", name: "flag hungary", cat: "Flags" },
    Emoji { glyph: "🇮🇩", name: "flag indonesia", cat: "Flags" },
    Emoji { glyph: "🇮🇲", name: "flag isle of man", cat: "Flags" },
    Emoji { glyph: "🇮🇳", name: "flag india", cat: "Flags" },
    Emoji { glyph: "🇮🇴", name: "flag british indian ocean territory", cat: "Flags" },
    Emoji { glyph: "🇮🇪", name: "flag ireland", cat: "Flags" },
    Emoji { glyph: "🇮🇷", name: "flag iran islamic republic of", cat: "Flags" },
    Emoji { glyph: "🇮🇶", name: "flag iraq", cat: "Flags" },
    Emoji { glyph: "🇮🇸", name: "flag iceland", cat: "Flags" },
    Emoji { glyph: "🇮🇱", name: "flag israel", cat: "Flags" },
    Emoji { glyph: "🇮🇹", name: "flag italy", cat: "Flags" },
    Emoji { glyph: "🇯🇲", name: "flag jamaica", cat: "Flags" },
    Emoji { glyph: "🇯🇪", name: "flag jersey", cat: "Flags" },
    Emoji { glyph: "🇯🇴", name: "flag jordan", cat: "Flags" },
    Emoji { glyph: "🇯🇵", name: "flag japan", cat: "Flags" },
    Emoji { glyph: "🇰🇿", name: "flag kazakhstan", cat: "Flags" },
    Emoji { glyph: "🇰🇪", name: "flag kenya", cat: "Flags" },
    Emoji { glyph: "🇰🇬", name: "flag kyrgyzstan", cat: "Flags" },
    Emoji { glyph: "🇰🇭", name: "flag cambodia", cat: "Flags" },
    Emoji { glyph: "🇰🇮", name: "flag kiribati", cat: "Flags" },
    Emoji { glyph: "🇰🇳", name: "flag saint kitts and nevis", cat: "Flags" },
    Emoji { glyph: "🇰🇷", name: "flag korea republic of", cat: "Flags" },
    Emoji { glyph: "🇰🇼", name: "flag kuwait", cat: "Flags" },
    Emoji { glyph: "🇱🇦", name: "flag lao people s democratic republic", cat: "Flags" },
    Emoji { glyph: "🇱🇧", name: "flag lebanon", cat: "Flags" },
    Emoji { glyph: "🇱🇷", name: "flag liberia", cat: "Flags" },
    Emoji { glyph: "🇱🇾", name: "flag libya", cat: "Flags" },
    Emoji { glyph: "🇱🇨", name: "flag saint lucia", cat: "Flags" },
    Emoji { glyph: "🇱🇮", name: "flag liechtenstein", cat: "Flags" },
    Emoji { glyph: "🇱🇰", name: "flag sri lanka", cat: "Flags" },
    Emoji { glyph: "🇱🇸", name: "flag lesotho", cat: "Flags" },
    Emoji { glyph: "🇱🇹", name: "flag lithuania", cat: "Flags" },
    Emoji { glyph: "🇱🇺", name: "flag luxembourg", cat: "Flags" },
    Emoji { glyph: "🇱🇻", name: "flag latvia", cat: "Flags" },
    Emoji { glyph: "🇲🇴", name: "flag macao", cat: "Flags" },
    Emoji { glyph: "🇲🇫", name: "flag saint martin french part", cat: "Flags" },
    Emoji { glyph: "🇲🇦", name: "flag morocco", cat: "Flags" },
    Emoji { glyph: "🇲🇨", name: "flag monaco", cat: "Flags" },
    Emoji { glyph: "🇲🇩", name: "flag moldova republic of", cat: "Flags" },
    Emoji { glyph: "🇲🇬", name: "flag madagascar", cat: "Flags" },
    Emoji { glyph: "🇲🇻", name: "flag maldives", cat: "Flags" },
    Emoji { glyph: "🇲🇽", name: "flag mexico", cat: "Flags" },
    Emoji { glyph: "🇲🇭", name: "flag marshall islands", cat: "Flags" },
    Emoji { glyph: "🇲🇰", name: "flag north macedonia", cat: "Flags" },
    Emoji { glyph: "🇲🇱", name: "flag mali", cat: "Flags" },
    Emoji { glyph: "🇲🇹", name: "flag malta", cat: "Flags" },
    Emoji { glyph: "🇲🇲", name: "flag myanmar", cat: "Flags" },
    Emoji { glyph: "🇲🇪", name: "flag montenegro", cat: "Flags" },
    Emoji { glyph: "🇲🇳", name: "flag mongolia", cat: "Flags" },
    Emoji { glyph: "🇲🇵", name: "flag northern mariana islands", cat: "Flags" },
    Emoji { glyph: "🇲🇿", name: "flag mozambique", cat: "Flags" },
    Emoji { glyph: "🇲🇷", name: "flag mauritania", cat: "Flags" },
    Emoji { glyph: "🇲🇸", name: "flag montserrat", cat: "Flags" },
    Emoji { glyph: "🇲🇶", name: "flag martinique", cat: "Flags" },
    Emoji { glyph: "🇲🇺", name: "flag mauritius", cat: "Flags" },
    Emoji { glyph: "🇲🇼", name: "flag malawi", cat: "Flags" },
    Emoji { glyph: "🇲🇾", name: "flag malaysia", cat: "Flags" },
    Emoji { glyph: "🇾🇹", name: "flag mayotte", cat: "Flags" },
    Emoji { glyph: "🇳🇦", name: "flag namibia", cat: "Flags" },
    Emoji { glyph: "🇳🇨", name: "flag new caledonia", cat: "Flags" },
    Emoji { glyph: "🇳🇪", name: "flag niger", cat: "Flags" },
    Emoji { glyph: "🇳🇫", name: "flag norfolk island", cat: "Flags" },
    Emoji { glyph: "🇳🇬", name: "flag nigeria", cat: "Flags" },
    Emoji { glyph: "🇳🇮", name: "flag nicaragua", cat: "Flags" },
    Emoji { glyph: "🇳🇺", name: "flag niue", cat: "Flags" },
    Emoji { glyph: "🇳🇱", name: "flag netherlands", cat: "Flags" },
    Emoji { glyph: "🇳🇴", name: "flag norway", cat: "Flags" },
    Emoji { glyph: "🇳🇵", name: "flag nepal", cat: "Flags" },
    Emoji { glyph: "🇳🇷", name: "flag nauru", cat: "Flags" },
    Emoji { glyph: "🇳🇿", name: "flag new zealand", cat: "Flags" },
    Emoji { glyph: "🇴🇲", name: "flag oman", cat: "Flags" },
    Emoji { glyph: "🇵🇰", name: "flag pakistan", cat: "Flags" },
    Emoji { glyph: "🇵🇦", name: "flag panama", cat: "Flags" },
    Emoji { glyph: "🇵🇳", name: "flag pitcairn", cat: "Flags" },
    Emoji { glyph: "🇵🇪", name: "flag peru", cat: "Flags" },
    Emoji { glyph: "🇵🇭", name: "flag philippines", cat: "Flags" },
    Emoji { glyph: "🇵🇼", name: "flag palau", cat: "Flags" },
    Emoji { glyph: "🇵🇬", name: "flag papua new guinea", cat: "Flags" },
    Emoji { glyph: "🇵🇱", name: "flag poland", cat: "Flags" },
    Emoji { glyph: "🇵🇷", name: "flag puerto rico", cat: "Flags" },
    Emoji { glyph: "🇰🇵", name: "flag korea democratic people s republic of", cat: "Flags" },
    Emoji { glyph: "🇵🇹", name: "flag portugal", cat: "Flags" },
    Emoji { glyph: "🇵🇾", name: "flag paraguay", cat: "Flags" },
    Emoji { glyph: "🇵🇸", name: "flag palestine state of", cat: "Flags" },
    Emoji { glyph: "🇵🇫", name: "flag french polynesia", cat: "Flags" },
    Emoji { glyph: "🇶🇦", name: "flag qatar", cat: "Flags" },
    Emoji { glyph: "🇷🇪", name: "flag r union", cat: "Flags" },
    Emoji { glyph: "🇷🇴", name: "flag romania", cat: "Flags" },
    Emoji { glyph: "🇷🇺", name: "flag russian federation", cat: "Flags" },
    Emoji { glyph: "🇷🇼", name: "flag rwanda", cat: "Flags" },
    Emoji { glyph: "🇸🇦", name: "flag saudi arabia", cat: "Flags" },
    Emoji { glyph: "🇸🇩", name: "flag sudan", cat: "Flags" },
    Emoji { glyph: "🇸🇳", name: "flag senegal", cat: "Flags" },
    Emoji { glyph: "🇸🇬", name: "flag singapore", cat: "Flags" },
    Emoji { glyph: "🇬🇸", name: "flag south georgia and the south sandwich islands", cat: "Flags" },
    Emoji { glyph: "🇸🇭", name: "flag saint helena ascension and tristan da cunha", cat: "Flags" },
    Emoji { glyph: "🇸🇯", name: "flag svalbard and jan mayen", cat: "Flags" },
    Emoji { glyph: "🇸🇧", name: "flag solomon islands", cat: "Flags" },
    Emoji { glyph: "🇸🇱", name: "flag sierra leone", cat: "Flags" },
    Emoji { glyph: "🇸🇻", name: "flag el salvador", cat: "Flags" },
    Emoji { glyph: "🇸🇲", name: "flag san marino", cat: "Flags" },
    Emoji { glyph: "🇸🇴", name: "flag somalia", cat: "Flags" },
    Emoji { glyph: "🇵🇲", name: "flag saint pierre and miquelon", cat: "Flags" },
    Emoji { glyph: "🇷🇸", name: "flag serbia", cat: "Flags" },
    Emoji { glyph: "🇸🇸", name: "flag south sudan", cat: "Flags" },
    Emoji { glyph: "🇸🇹", name: "flag sao tome and principe", cat: "Flags" },
    Emoji { glyph: "🇸🇷", name: "flag suriname", cat: "Flags" },
    Emoji { glyph: "🇸🇰", name: "flag slovakia", cat: "Flags" },
    Emoji { glyph: "🇸🇮", name: "flag slovenia", cat: "Flags" },
    Emoji { glyph: "🇸🇪", name: "flag sweden", cat: "Flags" },
    Emoji { glyph: "🇸🇿", name: "flag eswatini", cat: "Flags" },
    Emoji { glyph: "🇸🇽", name: "flag sint maarten dutch part", cat: "Flags" },
    Emoji { glyph: "🇸🇨", name: "flag seychelles", cat: "Flags" },
    Emoji { glyph: "🇸🇾", name: "flag syrian arab republic", cat: "Flags" },
    Emoji { glyph: "🇹🇨", name: "flag turks and caicos islands", cat: "Flags" },
    Emoji { glyph: "🇹🇩", name: "flag chad", cat: "Flags" },
    Emoji { glyph: "🇹🇬", name: "flag togo", cat: "Flags" },
    Emoji { glyph: "🇹🇭", name: "flag thailand", cat: "Flags" },
    Emoji { glyph: "🇹🇯", name: "flag tajikistan", cat: "Flags" },
    Emoji { glyph: "🇹🇰", name: "flag tokelau", cat: "Flags" },
    Emoji { glyph: "🇹🇲", name: "flag turkmenistan", cat: "Flags" },
    Emoji { glyph: "🇹🇱", name: "flag timor leste", cat: "Flags" },
    Emoji { glyph: "🇹🇴", name: "flag tonga", cat: "Flags" },
    Emoji { glyph: "🇹🇹", name: "flag trinidad and tobago", cat: "Flags" },
    Emoji { glyph: "🇹🇳", name: "flag tunisia", cat: "Flags" },
    Emoji { glyph: "🇹🇷", name: "flag t rkiye", cat: "Flags" },
    Emoji { glyph: "🇹🇻", name: "flag tuvalu", cat: "Flags" },
    Emoji { glyph: "🇹🇼", name: "flag taiwan province of china", cat: "Flags" },
    Emoji { glyph: "🇹🇿", name: "flag tanzania united republic of", cat: "Flags" },
    Emoji { glyph: "🇺🇬", name: "flag uganda", cat: "Flags" },
    Emoji { glyph: "🇺🇦", name: "flag ukraine", cat: "Flags" },
    Emoji { glyph: "🇺🇲", name: "flag united states minor outlying islands", cat: "Flags" },
    Emoji { glyph: "🇺🇾", name: "flag uruguay", cat: "Flags" },
    Emoji { glyph: "🇺🇸", name: "flag united states", cat: "Flags" },
    Emoji { glyph: "🇺🇿", name: "flag uzbekistan", cat: "Flags" },
    Emoji { glyph: "🇻🇦", name: "flag holy see vatican city state", cat: "Flags" },
    Emoji { glyph: "🇻🇨", name: "flag saint vincent and the grenadines", cat: "Flags" },
    Emoji { glyph: "🇻🇪", name: "flag venezuela bolivarian republic of", cat: "Flags" },
    Emoji { glyph: "🇻🇬", name: "flag virgin islands british", cat: "Flags" },
    Emoji { glyph: "🇻🇮", name: "flag virgin islands u s", cat: "Flags" },
    Emoji { glyph: "🇻🇳", name: "flag viet nam", cat: "Flags" },
    Emoji { glyph: "🇻🇺", name: "flag vanuatu", cat: "Flags" },
    Emoji { glyph: "🇼🇫", name: "flag wallis and futuna", cat: "Flags" },
    Emoji { glyph: "🇼🇸", name: "flag samoa", cat: "Flags" },
    Emoji { glyph: "🇾🇪", name: "flag yemen", cat: "Flags" },
    Emoji { glyph: "🇿🇦", name: "flag south africa", cat: "Flags" },
    Emoji { glyph: "🇿🇲", name: "flag zambia", cat: "Flags" },
    Emoji { glyph: "🇿🇼", name: "flag zimbabwe", cat: "Flags" },

    // Symbols
    Emoji { glyph: "#️⃣", name: "keycap hash", cat: "Symbols" },
    Emoji { glyph: "*️⃣", name: "keycap asterisk", cat: "Symbols" },
    Emoji { glyph: "0️⃣", name: "keycap 0", cat: "Symbols" },
    Emoji { glyph: "1️⃣", name: "keycap 1", cat: "Symbols" },
    Emoji { glyph: "2️⃣", name: "keycap 2", cat: "Symbols" },
    Emoji { glyph: "3️⃣", name: "keycap 3", cat: "Symbols" },
    Emoji { glyph: "4️⃣", name: "keycap 4", cat: "Symbols" },
    Emoji { glyph: "5️⃣", name: "keycap 5", cat: "Symbols" },
    Emoji { glyph: "6️⃣", name: "keycap 6", cat: "Symbols" },
    Emoji { glyph: "7️⃣", name: "keycap 7", cat: "Symbols" },
    Emoji { glyph: "8️⃣", name: "keycap 8", cat: "Symbols" },
    Emoji { glyph: "9️⃣", name: "keycap 9", cat: "Symbols" },

    // People
    Emoji { glyph: "👨‍👩‍👧‍👦", name: "family man woman girl boy", cat: "People" },
    Emoji { glyph: "👨‍👩‍👦", name: "family man woman boy", cat: "People" },
    Emoji { glyph: "👨‍👩‍👧", name: "family man woman girl", cat: "People" },
    Emoji { glyph: "👩‍👩‍👧‍👦", name: "family two women girl boy", cat: "People" },
    Emoji { glyph: "👨‍👨‍👧‍👦", name: "family two men girl boy", cat: "People" },
    Emoji { glyph: "👩‍👧", name: "family woman girl", cat: "People" },
    Emoji { glyph: "👨‍👦", name: "family man boy", cat: "People" },
    Emoji { glyph: "👩‍👦", name: "family woman boy", cat: "People" },
    Emoji { glyph: "👨‍👧", name: "family man girl", cat: "People" },
    Emoji { glyph: "👩‍👧‍👦", name: "family woman girl boy", cat: "People" },
    Emoji { glyph: "👨‍👧‍👦", name: "family man girl boy", cat: "People" },
    Emoji { glyph: "🧑‍💻", name: "technologist developer coder computer", cat: "People" },
    Emoji { glyph: "👩‍💻", name: "woman technologist developer coder", cat: "People" },
    Emoji { glyph: "👨‍💻", name: "man technologist developer coder", cat: "People" },
    Emoji { glyph: "🧑‍🎓", name: "student graduation", cat: "People" },
    Emoji { glyph: "🧑‍🏫", name: "teacher education", cat: "People" },
    Emoji { glyph: "🧑‍⚕️", name: "health worker doctor", cat: "People" },
    Emoji { glyph: "🧑‍🚀", name: "astronaut space", cat: "People" },
    Emoji { glyph: "🧑‍🍳", name: "cook chef", cat: "People" },
    Emoji { glyph: "🧑‍🔬", name: "scientist lab", cat: "People" },
    Emoji { glyph: "🧑‍🎨", name: "artist painter", cat: "People" },
    Emoji { glyph: "🧑‍🚒", name: "firefighter", cat: "People" },
    Emoji { glyph: "🧑‍✈️", name: "pilot", cat: "People" },
    Emoji { glyph: "🧑‍🌾", name: "farmer", cat: "People" },
    Emoji { glyph: "🧑‍🔧", name: "mechanic", cat: "People" },
    Emoji { glyph: "🧑‍⚖️", name: "judge", cat: "People" },

    // Symbols
    Emoji { glyph: "🏳️‍🌈", name: "rainbow pride flag", cat: "Symbols" },
    Emoji { glyph: "🏳️‍⚧️", name: "transgender pride flag", cat: "Symbols" },
    Emoji { glyph: "🏴‍☠️", name: "pirate flag", cat: "Symbols" },
    Emoji { glyph: "❤️‍🔥", name: "heart fire burning love", cat: "Symbols" },
    Emoji { glyph: "❤️‍🩹", name: "mending heart healing", cat: "Symbols" },

    // People
    Emoji { glyph: "👩‍❤️‍👨", name: "couple woman man heart", cat: "People" },
    Emoji { glyph: "👨‍❤️‍👨", name: "couple men heart", cat: "People" },
    Emoji { glyph: "👩‍❤️‍👩", name: "couple women heart", cat: "People" },
    Emoji { glyph: "🧑‍🦽", name: "person manual wheelchair", cat: "People" },
    Emoji { glyph: "🧑‍🦼", name: "person motorized wheelchair", cat: "People" },
    Emoji { glyph: "🧑‍🦯", name: "person white cane", cat: "People" },
    Emoji { glyph: "👨‍🦯", name: "man white cane", cat: "People" },
    Emoji { glyph: "👩‍🦯", name: "woman white cane", cat: "People" },
    Emoji { glyph: "👨‍🦽", name: "man wheelchair", cat: "People" },
    Emoji { glyph: "👩‍🦽", name: "woman wheelchair", cat: "People" },
    Emoji { glyph: "👨‍🦼", name: "man motorized wheelchair", cat: "People" },
    Emoji { glyph: "👩‍🦼", name: "woman motorized wheelchair", cat: "People" },
];
