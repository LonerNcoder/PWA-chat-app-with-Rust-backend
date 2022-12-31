import{_ as M}from"./login.07f597ac.js";import{a as j,r as w,b as $,o as u,e as _,u as p,f as a,t as C,h as b,i as F,p as q,j as k,k as O,F as N,l as B,w as T,v as D,m as R,q as z,s as H,x as J,y as P,z as I,c as W}from"./entry.c85a5a66.js";const X=["src"],G={__name:"dicebear",props:["name"],setup(t){const e=t,n=w("");return $(()=>{n.value=`https://avatars.dicebear.com/api/adventurer/${e.name}.svg`}),(r,s)=>(u(),_("img",{src:p(n),alt:"Image"},null,8,X))}},K=j(G,[["__scopeId","data-v-786dadb2"]]);const Q={class:"bg-blue-500 w-[45px] h-[45px] flex items-center justify-center rounded-full"},Y={class:"font-bold text-sm text-white"},Z={__name:"avatar",props:["name","main"],setup(t){const e=t,n=w("");function r(s=""){if(s.includes(" ")){const l=s.split(" ");return`${l[0].charAt(0)}${l[1].charAt(0)}`.toUpperCase()}return`${s.slice(0,2)}`.toUpperCase()}return $(()=>{n.value=r(e.name)}),(s,l)=>(u(),_("div",Q,[a("span",Y,C(p(n)),1)]))}},ee=j(Z,[["__scopeId","data-v-27785c8f"]]),te={class:""},ne={key:0},se={class:"w-full flex justify-end"},re={class:"flex gap-3 justify-end mt-4"},oe={class:"max-w-[65%] bg-violet-500 p-3 text-sm rounded-xl rounded-br-none"},ae={class:"text-white"},ce={key:1},ie={class:"mt-auto"},le={class:"max-w-[65%] bg-gray-200 p-3 text-sm rounded-xl rounded-bl-none"},de={key:"{content+username}"},ue={__name:"blob",props:["msg","right","sender_name","receiver_name"],setup(t){const e=t;return(n,r)=>{const s=ee;return u(),_("div",te,[t.right?(u(),_("div",ne,[a("div",se,[a("div",re,[a("div",oe,[a("p",ae,C(e.msg.content),1)]),(u(),_("div",{class:"mt-auto",key:e.msg.created_at},[b(s,{name:e.sender_name},null,8,["name"])]))])])])):(u(),_("div",ce,[(u(),_("div",{class:"flex gap-3 w-full fucker mt-4",key:e.msg.created_at},[a("div",ie,[b(s,{name:e.receiver_name},null,8,["name"])]),a("div",le,[a("p",de,C(e.msg.content),1)])]))]))])}}};let S;const _e=new Uint8Array(16);function me(){if(!S&&(S=typeof crypto<"u"&&crypto.getRandomValues&&crypto.getRandomValues.bind(crypto),!S))throw new Error("crypto.getRandomValues() not supported. See https://github.com/uuidjs/uuid#getrandomvalues-not-supported");return S(_e)}const d=[];for(let t=0;t<256;++t)d.push((t+256).toString(16).slice(1));function pe(t,e=0){return(d[t[e+0]]+d[t[e+1]]+d[t[e+2]]+d[t[e+3]]+"-"+d[t[e+4]]+d[t[e+5]]+"-"+d[t[e+6]]+d[t[e+7]]+"-"+d[t[e+8]]+d[t[e+9]]+"-"+d[t[e+10]]+d[t[e+11]]+d[t[e+12]]+d[t[e+13]]+d[t[e+14]]+d[t[e+15]]).toLowerCase()}const ve=typeof crypto<"u"&&crypto.randomUUID&&crypto.randomUUID.bind(crypto),E={randomUUID:ve};function ge(t,e,n){if(E.randomUUID&&!e&&!t)return E.randomUUID();t=t||{};const r=t.random||(t.rng||me)();if(r[6]=r[6]&15|64,r[8]=r[8]&63|128,e){n=n||0;for(let s=0;s<16;++s)e[n+s]=r[s];return e}return pe(r)}const V=F("root",{state:()=>({user:{id:null,username:null,logged_in:!1},friends:[],conversations:{}}),getters:{getUser:t=>t.user,getFriends:t=>t.friends,getConversations:(t,e)=>t.conversations[e]},actions:{setUser:(t,e)=>{t.user=e},addFriends:(t,e)=>{let n=0;for(let r of t.friends)r.id===e.id&&(n=1);n===0&&t.friends.push(e)},check:t=>{console.log("checked")}},persist:{storage:q.localStorage}}),he={class:"bg-grey h-[100vh] min-w-[60%] overflow-scroll pb-[80px]"},fe={class:"flex flex-row bg-violet-700 h-20 justify-between items-center rounded-l-2xl sticky top-0"},ye={class:"scroll-div"},xe={class:"parag pl-2 pr-2"},be={id:"input-div",class:"absolute bottom-4 w-[60%] flex flex-row justify-between items-center"},we={__name:"Message",props:{sender:{type:Object,required:!0},receiver:{type:Object,reequired:!0}},emits:["conversation_ended"],setup(t,{emit:e}){const n=t,r="https://paragme.pagekite.me",s=V();w(0);const l=()=>{e("conversation_ended")},c=w(""),f=async()=>{const i={id:ge(),chat_type:"TEXT",value:[c.value],room_id:"new room",sender_id:n.sender.id,receiver_id:n.receiver.id};try{s.conversations[n.receiver.id].push({content:c.value,sender_id:n.sender.id,created_at:new Date().toISOString()})}catch{s.conversations[n.receiver.id]=[],s.conversations[n.receiver.id].push({content:c.value,sender_id:n.sender.id,receiver_id:n.receiver.id})}await h(JSON.stringify(i)),c.value="",document.getElementsByClassName("scroll-div")[0].lastElementChild.scrollIntoView({behavior:"smooth"})},m=k({current:null}),h=i=>{m.current!=null&&m.current.send(i)};$(async()=>{y(),m.current!=null&&m.current.close();const i="wss://paragme.pagekite.me/"+n.sender.id+"/"+n.receiver.id+"/ws";return m.current=new WebSocket(i),m.current.onopen=()=>console.log("ws opened"),m.current.onclose=()=>console.log("ws closed"),m.current.onmessage=async g=>{await y(),document.getElementsByClassName("scroll-div")[0].lastElementChild.scrollIntoView({behavior:"smooth"})},!0});async function y(){let i=`${r}/latest/${n.sender.id}/${n.receiver.id}`;try{let o=await(await fetch(i)).json();if(o.length>0)for(let v of o)try{s.conversations[n.receiver.id].push(v)}catch{s.conversations[n.receiver.id]=[],s.conversations[n.receiver.id].push(v)}}catch{console.log("no conversation yet or no unseen pending message")}return document.getElementsByClassName("scroll-div")[0].lastElementChild.scrollIntoView({behavior:"smooth"}),!0}return O(()=>{console.log("unmounted"),m.current.close()}),(i,g)=>{const o=K,v=ue;return u(),_("div",he,[a("div",fe,[a("button",{class:"bg-white text-violet-600 ml-5 px-4 py-1 rounded-lg",onClick:l}," Back"),a("div",null,[b(o,{name:n.receiver.name},null,8,["name"])])]),a("div",ye,[(u(!0),_(N,null,B(p(s).conversations[n.receiver.id],x=>(u(),_("div",{key:x.id},[a("div",xe,[b(v,{msg:x,right:x.sender_id==n.sender.id,sender_name:n.sender.name,receiver_name:n.receiver.name},null,8,["msg","right","sender_name","receiver_name"])])]))),128))]),a("div",be,[T(a("input",{"onUpdate:modelValue":g[0]||(g[0]=x=>R(c)?c.value=x:null),type:"text",placeholder:"Type Something...",class:"w-[100%] border-b-2 px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-green-600"},null,512),[[D,p(c)]]),a("button",{onClick:f},"Send")])])}}},ke=z({name:"ClientOnly",inheritAttrs:!1,props:["fallback","placeholder","placeholderTag","fallbackTag"],setup(t,{slots:e,attrs:n}){const r=w(!1);return $(()=>{r.value=!0}),s=>{var m;if(r.value)return(m=e.default)==null?void 0:m.call(e);const l=e.fallback||e.placeholder;if(l)return l();const c=s.fallback||s.placeholder||"",f=s.fallbackTag||s.placeholderTag||"span";return _(f,n,c)}}});const $e={class:"h-[10rem] flex justify-between items-center pl-5 pr-5"},Ue={class:"logo"},Ce={class:"flex flex-row gap-x-5"},Se={class:"friend_list flex flex-col justify-start items-center"},Ie=["onClick"],je={__name:"searchUser",props:{sender:{type:Object,required:!0}},emits:["found","conversation_started","not_found"],setup(t,{emit:e}){const n=t,r="https://paragme.pagekite.me",s=V(),l=w(""),c=k([]);$(async()=>{try{c.push(...s.friends),console.log(c);return}catch(h){console.log(h)}});const f=async()=>{let h=`${r}/users/phone/${l.value}`;try{let i=await(await fetch(h)).json(),g=0;if(i.id){for(let o of c)o.id===i.id&&(g=1);if(g==0){c.push(i),s.friends.push(i),l.value="",e("found",i);return}}else{e("not_found");return}}catch{e("not_found");return}},m=h=>{e("conversation_started",h)};return(h,y)=>(u(),_("div",null,[a("div",$e,[a("div",Ue,C(n.sender.name),1),a("div",Ce,[T(a("input",{"onUpdate:modelValue":y[0]||(y[0]=i=>R(l)?l.value=i:null),type:"text",class:"mt-4 rounded-lg",placeholder:"Friend's Phone Number"},null,512),[[D,p(l)]]),a("button",{onClick:f,class:"px-6 py-2 mt-4 text-white bg-violet-600 rounded-lg hover:bg-violet-700"},"Search")])]),a("div",Se,[(u(!0),_(N,null,B(p(c),i=>(u(),_("div",{key:i.id,class:"friend_list_card",onClick:g=>m(i)},C(i.username),9,Ie))),128))])]))}},Ve=j(je,[["__scopeId","data-v-e811de0b"]]);const Ee={key:0,class:"h-[100%] w-[100%] z-[1000]"},Ne={key:1,class:"flex flex-row"},Be={class:"bg-slate-400 h-[100vh] w-[40%] overflow-scroll"},Re={__name:"index",async setup(t){let e,n;const r=([e,n]=H(()=>V()),e=await e,n(),e);k({count:0,name:"",searchedName:"",phone:"",email:"",searched:!1,res:"",terms:{}});const s=w(!0),l=k({id:0,name:""}),c=k({id:0,name:""}),f=k({started:!1});function m(o){r.user.id=o.id,r.user.username=o.username,r.user.logged_in=!0,h(o)}function h(o){l.id=o.id,l.name=o.username}function y(o){c.id=o.id,c.name=o.username}function i(o){c.id=o.id,c.name=o.username,f.started=!1,f.started=!0}$(async()=>{try{let o=await r.getUser;o.logged_in?(h(o),s.value=r.getUser.logged_in):s.value=!1}catch{s.value=!1}return 0}),J(r.getUser,()=>{s.value=r.getUser.logged_in});function g(){alert("user does not exist! please ask them to join ")}return(o,v)=>{const x=M,A=we,L=ke;return u(),_("div",null,[b(L,null,{default:P(()=>[p(r).user.logged_in?I("",!0):(u(),_("div",Ee,[b(x,{onLogged_in:v[0]||(v[0]=U=>m(U))})])),p(r).user.logged_in?(u(),_("div",Ne,[a("div",Be,[b(Ve,{sender:p(l),onFound:v[1]||(v[1]=U=>y(U)),onNot_found:g,onConversation_started:v[2]||(v[2]=U=>i(U))},null,8,["sender"])]),p(f).started?(u(),W(A,{sender:p(l),receiver:p(c),onConversation_ended:v[3]||(v[3]=()=>{p(f).started=!1}),key:p(c).id},null,8,["sender","receiver"])):I("",!0)])):I("",!0)]),_:1})])}}};export{Re as default};
