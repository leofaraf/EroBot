import {useEffect, useState} from "react";
import img from "../assets/other/img.png";
import famale from "../assets/logo/famale.svg";
import tiktok from "../assets/logo/tiktok.svg";
import instagram from "../assets/logo/instagram.svg";
import twitch from "../assets/logo/twitch.svg";
import onlyfans from "../assets/logo/onlyfans.svg";
import boosty from "../assets/logo/boosty.svg";
import strawberry from "../assets/logo/strawberry.svg";
import video_placeholder from "../assets/other/images.png"

function Router() {

    const [models, setModels] = useState([]);
    const [category, setCategory] = useState("all");
    const [user, setUser] = useState(null)
    const [model, setModel] = useState(null);
    const [is_only_vip, setOnlyVip] = useState(false);
    const [topbarModels, setTopbapModels] = useState(null);
    const [post, setPost] = useState(null);
    const [bg, setBg] = useState("bg-white"); //bg-zinc-800
    const [secondColor, setSecondColor] = useState("text-black") //text-white

    useEffect(() => {
        const script = document.createElement("script")

        script.src =
            "https://telegram.org/js/telegram-web-app.js"

        document.body.appendChild(script)
    }, [])

    useEffect(() => {
        console.log(window)
        if (window?.Telegram?.WebApp?.themeParams?.bg_color) {
            setBg("bg-[" + window.Telegram.WebApp.themeParams.bg_color + "]")
        }
        if (window?.Telegram?.WebApp?.colorScheme && window.Telegram.WebApp.colorScheme !== "light") {
            setSecondColor("text-white")
        }
    }, [window.Telegram])

    useEffect(() => {
        fetch("/api/models")
            .then(r => {
                if (r.status === 200) {
                    r.json().then(json => {
                        setModels(json);
                    })
                }
            });
        const searchParams = new URLSearchParams(window.location.search);
        if (searchParams.has("id")) {
            fetch("api/user/" + searchParams.get("id"))
                .catch(e => console.log("there isn't " + e))
                .then(r => {
                    if (r.status === 200) {
                        r.json().then(json => {
                            setUser(json)
                        })
                    } else {
                        setUser(null) // need for run background hook
                    }
                })
        }
    }, []);

    useEffect(() => {
        if (models) {
            setTopbapModels(getRandomElements(models, 5));
        }
    }, [models]);

    const open_model = (model) => {
        setModel(model);
    }

    const close_model = () => {
        setModel(null);
    }

    const filter_models = (models) => {
        if (category === "all") {
            return models
        }
        const filtered_models = []
        models.map(model => {
            if (model.category === category) {
                filtered_models.push(model)
            }
        })
        return filtered_models
    }

    const filter_posts = (posts) => {
        if (is_only_vip === false) {
            return posts
        }
        const filtered_models = []
        posts.map(post => {
            if (post.is_vip === true) {
                filtered_models.push(posts)
            }
        })
        return filtered_models
    }

    function getRandomElements(arr, numOfElements) {
        let result = [];
        let tempArr = [...arr]; // Clone the array to not mutate the original one

        // Shuffle array using the Fisher-Yates (Durstenfeld) shuffle
        for (let i = tempArr.length - 1; i > 0; i--) {
            const j = Math.floor(Math.random() * (i + 1));
            [tempArr[i], tempArr[j]] = [tempArr[j], tempArr[i]]; // Swap elements
        }

        // Get the first 'numOfElements' elements after shuffle, or all if there are less than 'numOfElements'
        result = tempArr.slice(0, numOfElements < arr.length ? numOfElements : arr.length);

        return result;
    }

    return <div className={`${"bg-[#1a313f]"} ${secondColor} h-screen p-4 gap-2 text-sm`}>
        <p>{bg}, {secondColor}</p>
        {model ? (
            <div className={"flex flex-col gap-3"}>
                {post ? (
                    <>
                        {!(post.is_vip && !user) ? (
                            <div className={[bg, "w-full h-screen flex flex-col p-4"].join(" ")}>
                                <div className={"flex justify-between text-lg"}>
                                    <p>{post.name}</p>
                                    <button
                                        onClick={() => setPost(null)}><u>Выйти</u></button>
                                </div>
                                <div className={"relative w-full h-full flex flex-col justify-center"}>
                                    {post?.media?.media_type === "Image" ? (
                                        <img className={"object-cover"} src={post.media.path}/>
                                    ) : (
                                        <video controls autoPlay name="media">
                                            <source src={post.media.path} type={"video/mp4"}/>
                                        </video>
                                    )}
                                </div>
                            </div>
                        ) : (
                            <div className={[bg, "w-full h-screen flex flex-col p-4"].join(" ")}>
                                <div className={"flex justify-between items-center pb-2 text-lg"}>
                                    <p>{post.name}</p>
                                    <button
                                        onClick={() => setPost(null)}><u>Выйти</u></button>
                                </div>
                                <p className={"text-2xl px-2 flex flex-col justify-center h-full"}>
                                    У вас не VIP статус, вы не можете смотреть VIP-контент.
                                    Выйдите в чат и напишите /vip, чтобы получить его! ❤️
                                </p>
                            </div>
                        )}
                    </>
                ) : (
                    <>
                        <div className={"flex justify-between"}>
                            <p className={"text-xl"}>{model.name}</p>
                            <button
                                onClick={() => close_model()}><u>Назад</u></button>
                        </div>
                        <div className={
                            "flex justify-between items-center bg-gradient-to-r from-fuchsia-700 to-cyan-300 p-3 rounded-lg"
                        }>
                            <p>Отображать только <br/>интим фото/видео</p>
                            <div className="checkbox-wrapper-6">
                                <input value={is_only_vip} onChange={event =>
                                    setOnlyVip(event.target.checked)} className="tgl tgl-light" id="cb1-6" type="checkbox"/>
                                <label className="tgl-btn" htmlFor="cb1-6"/>
                            </div>
                        </div>
                        <div className={"grid grid-cols-3 gap-3"}>
                            {model.posts && model.posts.map(post => {
                                if (is_only_vip && !post.is_vip) {
                                    return null;
                                }

                                return (
                                    <>
                                        <div className={"w-full text-white relative rounded-xl h-44 flex justify-center"}
                                             onClick={() => setPost(post)}>
                                            {post.is_vip && (
                                                <>
                                                    <div className={"p-2 z-10 absolute top-0 left-0 flex justify-center"}>
                                                        <img src={strawberry}/>
                                                    </div>
                                                </>
                                            )}
                                            <img className={post.is_vip ? "rounded-xl blur-sm object-cover" :
                                                "rounded-xl object-cover"} src={post.media.media_type !== "Video" ? post.media.path : video_placeholder} />
                                            <div className={"rounded-b-xl bg-opacity-50 h-10 absolute bottom-0 w-full flex items-center justify-center bg-black"}>
                                                <p>{post.name}</p>
                                            </div>
                                        </div>
                                    </>
                                )
                            })}
                        </div>
                    </>
                )}
            </div>
        ) : (
            <>
                <div className="flex flex-row gap-5 overflow-x-auto">
                    {topbarModels && topbarModels.map(model => {
                        return (
                            <div onClick={() => open_model(model)}>
                                <div className="w-14 h-14 bg-gradient-to-b
                                from-pink-500 to-cyan-300
                                rounded-full p-1">
                                    <img className={"w-full h-full rounded-full"} src={model.media.path}/>
                                </div>
                                <p>{model.name}</p>
                            </div>
                        )
                    })}
                </div>

                <div className="flex justify-between w-ful pt-6">
                    <p>Выберите категорию</p>
                    <button className={"text"}
                            onClick={() => setCategory("all")}><u>Показать все</u></button>
                </div>
                <div className="grid grid-cols-3 gap-3 w-full pt-1">
                    <button className="category-button" onClick={() => setCategory("Influential")}>
                        <img src={famale} width={16} />Блогершы</button>
                    <button className="category-button" onClick={() => setCategory("Cosplay")}>
                        <img src={tiktok} width={16} />Косплеи</button>
                    <button className="category-button" onClick={() => setCategory("Star")}>
                        <img src={instagram} width={16} />Звезды</button>
                    <button className="category-button" onClick={() => setCategory("Twitch")}>
                        <img src={twitch} width={16} />Twitch</button>
                    <button className="category-button" onClick={() => setCategory("OnlyFans")}>
                        <img src={onlyfans} width={16} />OnlyFans</button>
                    <button className="category-button" onClick={() => setCategory("Boosty")}>
                        <img src={boosty} width={16} />Boosty</button>
                </div>
                <div className={"grid grid-cols-3 gap-3 pt-3"}>
                    {models ? filter_models(models).map(model => {
                        return (
                            <div className={"w-full text-white relative rounded-xl h-44 flex justify-center"}
                                 onClick={() => open_model(model)}>
                                <div className={"rounded-tr-xl rounded-bl-xl absolute top-0 right-0 flex px-1 justify-center bg-black"}>
                                    <p>{model?.posts?.length} фото</p>
                                </div>
                                <img className={"rounded-xl object-cover"} src={model.media.path} />
                                <div className={"rounded-b-xl bg-opacity-50 h-10 absolute bottom-0 w-full flex items-center justify-center bg-black"}>
                                    <p>{model.name}</p>
                                </div>
                            </div>
                        )
                    }) : (
                        <p>Почему-то произошла ошибка загрузки, обратитесь к администратору</p>
                    )}
                </div>
            </>
        )}
    </div>;
}

export default Router;
