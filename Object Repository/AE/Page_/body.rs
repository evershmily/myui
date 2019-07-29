<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body</name>
   <tag></tag>
   <elementGuidId>815a990e-af56-4f29-b7ee-c76f6e96583a</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
	
	
	
	
		
		正在通过北纬认证中心认证
		
			
			
			
			 记住用户名
		
		登录
		
			
			
			
		
		
	




(function(){
	'use strict';
	var $loginName=$('#loginArea input[name=&quot;loginName&quot;]'),
		$password=$('#loginArea input[name=&quot;password&quot;]'),
		$verification=$('#loginArea input[name=&quot;verification&quot;]');
	var accesskey=getData(&quot;accesskey&quot;);
	var redirecturl=getData(&quot;redirecturl&quot;);
	var projectname=getData(&quot;projectname&quot;);
	
	//绑定事件
	var bindEvents=function(){
		$('body')
		.on('click','.loginStyle a',function(){//切换登录方式
			localStorage.loginStyleMemory=$(this).attr(&quot;flag&quot;);
			// if($(this).attr('id')!=&quot;wechatLoginBtn&quot;){
			// 	$('.loginStyle a').removeClass('selected');
			// 	$(this).addClass('selected');
			// }
			$('.loginStyle a').removeClass('selected');
			$(this).addClass('selected');
		})
		.on('click','#wechatLoginBtn',function(){//展开微信码
			$('#IronCurtain').fadeIn();
			$('#QRCode').fadeIn();
		})
		.on('click','#IronCurtain',function(){//收起微信码
			$('#IronCurtain').fadeOut();
			$('#QRCode').fadeOut();
			$('#localLoginBtn').click();
		})
		.on('click','#loginBtn',function(){//按钮提交
			if($(this).hasClass('maskTag')){
				return;
			}
			if($verification.val()==&quot;&quot;&amp;&amp;localStorage.ers>0){
				$verification.addClass(&quot;red&quot;);
				return;
			}
			if($password.val()==&quot;&quot;){
				$password.addClass(&quot;red&quot;);
				return;
			}
			if($loginName.val()==&quot;&quot;){
				$loginName.addClass(&quot;red&quot;);
				return;
			}
			login($loginName.val(),$password.val(),$verification.val(),$(this).attr('sessionKey'),$('.loginStyle a.selected').attr('flag'));
		})
		.on('focus','input:text,input:password',function(){//焦点取消提示
			$(this).removeClass(&quot;red&quot;);
		})
		.on('blur','input:text,input:password',function(){//失点提示
			if($(this).val()==&quot;&quot;) $(this).addClass(&quot;red&quot;);
		})
		.on('click','.showPS',function(){//是否显示密码
			if($(this).hasClass(&quot;showshow&quot;)){
				$(this).removeClass(&quot;showshow&quot;);
				$(this).prev('input').attr(&quot;type&quot;,&quot;password&quot;);
			}else{
				$(this).addClass(&quot;showshow&quot;);
				$(this).prev('input').attr(&quot;type&quot;,&quot;text&quot;);
			}
		})
		.on('click','#verCode',function(){//换验证码
			lordCaptcha($(this).attr('sessionKey'));
		})
		.on('keypress','input',function(evt){//回车提交
			evt=(evt)?evt:((window.event)?window.event:&quot;&quot;);
			var key=evt.keyCode?evt.keyCode:evt.which;
			if(key==13){
				$('#loginBtn').click();
			}
		});
	};
	bindEvents();

	//填充验证码
	function lordCaptcha(sessionkey){
		var randomNum=Math.floor(Math.random()*10000);
		$('#verCode').attr(&quot;src&quot;,&quot;&quot;);
		$('#verCode').attr(&quot;src&quot;,'/authorcenter/author/getcaptcha/'+sessionkey+'?'+randomNum);
	};
	
	var aesKey='';
	//获取sessionKey
	function getSessionKey(accesskey,redirecturl){
		if(redirecturl){
			$.ajax({
				type:&quot;post&quot;,
				dataType:&quot;json&quot;,
				contentType:&quot;application/json&quot;,
				url:&quot;/authorcenter/author/getsessionkey&quot;,
				data:JSON.stringify({&quot;accesskey&quot;:accesskey,&quot;redirecturl&quot;:redirecturl}),
				success:function(data){
					if(data.code!=0){
						if(data.code==3){
							alert(&quot;accesskey有误或超时，准备退回至第三方平台&quot;);
							setTimeout(function(){
								window.location.replace(redirecturl);
							},1500);
						}else{
							alert(&quot;sessionKey获取失败，重新获取ING&quot;);
							setTimeout(function(){
								getSessionKey(accesskey,redirecturl);
							},1500);
						}
					}else{
						if(data.styletype==1){
							$(&quot;#cssList&quot;).attr(&quot;href&quot;,data.styleurl);
						}else{
							switchStyle(projectname); // 切定制样式
						}
						$('#waitStyle').fadeOut();
						$('.loginBox').fadeIn();
						switchLoginStyle(); // 切上一次登录方式
						checkRemember(); // 切默认记住用户名
						
						aesKey=data.aeskey;
						$('#loginBtn,#verCode').attr('sessionKey',data.sessionkey);
						$('#verCode').click();
						getAppIdInfo(data.sessionkey);
						goBackRedirecturl(redirecturl); //计时开始
						// wechatresult(data.sessionkey);
					}
				},
				error:function(){
					alert('sessionKey获取接口出现错误，尝试重新获取ING');
					// window.location.replace(getData(&quot;redirecturl&quot;));
					setTimeout(function(){
						getSessionKey(accesskey,redirecturl);
					},1500);
				}
			})
		}else{
			window.location.replace(&quot;authNotice.html&quot;);
		}
	};

	//获取微信认证信息
	function getAppIdInfo(sessionkey){
		$.ajax({
			type:&quot;post&quot;,
			dataType:&quot;json&quot;,
			contentType:&quot;application/json&quot;,
			url:&quot;/authorcenter/wechatlogin/getappidinfo&quot;,
			data:JSON.stringify({&quot;sessionkey&quot;:sessionkey}),
			success:function(data){
				if(data.code!=0){
					alert(data.msg);
				}else{
					if(data.wechatbutton==&quot;off&quot;){
						//do nothing...
					}else{
						window.WwLogin({
									&quot;id&quot;:&quot;QRCode&quot;,  
									&quot;appid&quot;:data.appid,
									&quot;agentid&quot;:data.agentid,
									&quot;redirect_uri&quot;:data.redirect_uri,
									&quot;state&quot;:sessionkey,
									&quot;href&quot;:&quot;&quot;
								});
					}
				}
			},
			error:function(){
				alert('获取微信信息不成功呢~');
			}
		})
	};

	//轮询接口
	// function wechatresult(sessionkey){
	// 	$.ajax({
	// 		type:&quot;post&quot;,
	// 		dataType:&quot;json&quot;,
	// 		contentType:&quot;application/json&quot;,
	// 		url:&quot;/authorcenter/wechatlogin/wechatresult&quot;,
	// 		data:JSON.stringify({&quot;sessionkey&quot;:sessionkey}),
	// 		success:function(data){
	// 			if(data.code!=0){
	// 				setTimeout(function(){
	// 					wechatresult(sessionkey);
	// 				},500);
	// 			}else{
	// 				if(data.token){
	// 					ugo(data.redirecturl,data.token);
	// 				}else{
	// 					setTimeout(function(){
	// 						wechatresult(sessionkey);
	// 					},500);
	// 				}
	// 			}
	// 		},
	// 		error:function(){
	// 			setTimeout(function(){
	// 				wechatresult(sessionkey);
	// 			},500);
	// 		}
	// 	})
	// }; 

	//Aes加密
	function encrypt (password, key, iv) {
		password = CryptoJS.enc.Utf8.parse(password)
		key = CryptoJS.enc.Hex.parse(key)
		iv = CryptoJS.enc.Hex.parse(iv)
		var encrypted = CryptoJS.AES.encrypt(password, key, {
			iv: iv,
			mode: CryptoJS.mode.CBC,
			padding: CryptoJS.pad.Pkcs7
		});
		return encrypted.ciphertext.toString(CryptoJS.enc.Hex).toUpperCase();
	};
	var repeat=function(char,l){
		if(typeof char !== 'string' || typeof l !== 'number') {
			throw Error(
			'function repeat(char: string, l: number): string {} invalid parameters'
			);
		}
		var s = '';
		while (l > 0) {
			s += char;
			l--;
		}
		return s;
	};

	//登录
	function login(loginname,password,captcha,sessionkey,type){
		$(&quot;#loginBtn&quot;).addClass('maskTag');
		// var obj={
		// 	code0:&quot;登录成功&quot;,
		// 	code1:&quot;用户不存在或者用户名密码错误&quot;,
		// 	code2:&quot;OA鉴权失败&quot;,
		// 	code3:&quot;accesskey有误&quot;,
		// 	code4:&quot;验证码错误&quot;,
		// 	code5:&quot;验签失败&quot;,
		// 	code6:&quot;解密失败&quot;,
		// 	code7:&quot;验证码失效&quot;,
		// 	code8:&quot;TOKEN超时&quot;,
		// 	code9:&quot;OA返回未知错误&quot;,
		// 	code10:&quot;OA没连上，你又是第一次登录，救不了你了&quot;,
		// 	code11:&quot;没这个平台key,请联系鉴权管理员&quot;,
		// 	code12:&quot;AES加密失败&quot;,
		// 	code13:&quot;鉴权平台用户列表空&quot;,
		// 	code14:&quot;鉴权平台用户列表空&quot;,
		// 	code15:&quot;登录解密或数据库异常&quot;,
		// }
		var evalkey = repeat(aesKey, 2);
		var evaliv = repeat('1', 32);
		var encryptedPwd = encrypt(password, evalkey, evaliv);
		$.ajax({
			type:&quot;post&quot;,
			dataType:&quot;json&quot;,
			contentType:&quot;application/json&quot;,
			url:&quot;/authorcenter/author/Login&quot;,
			data:JSON.stringify({&quot;loginname&quot;:loginname,&quot;password&quot;:encryptedPwd,&quot;captcha&quot;:captcha,&quot;sessionkey&quot;:sessionkey,&quot;type&quot;:type}),//'oa' 'local'
			success:function(data){
				$(&quot;#loginBtn&quot;).removeClass('maskTag');
				if(data.code!=0){
					// alert(obj[&quot;code&quot;+data.code]);
					alert(data.message);
					$('#verLi').show();
					$('#verCode').click();
					localStorage.ers=parseInt(localStorage.ers)+1;
				}else{
					if($('.remember').is(&quot;:hidden&quot;)){
						// do nothing
					}else{
						if($(&quot;#rememberCheck&quot;).is(&quot;:checked&quot;)){
							localStorage.username=loginname;
							localStorage.checker=&quot;true&quot;;
						}else{
							localStorage.username=&quot;&quot;;
							localStorage.checker=&quot;false&quot;;
						}
					}
					// 用户名密码登录后的方式留存
					localStorage.loginStyleMemory=type;
					ugo(redirecturl,data.token);
				}
			},
			error:function(){
				$(&quot;#loginBtn&quot;).removeClass('maskTag');
				alert('登录鉴权失败，再来次');
				window.location.replace(redirecturl);
			}
		})
	};

	//跳转页面
	function ugo(redirecturl,token){
		if(redirecturl.indexOf('?')===-1){
			window.location.replace(redirecturl+&quot;?token=&quot;+token);
		}else{
			if(redirecturl.indexOf('?')===redirecturl.length-1){
				window.location.replace(redirecturl+&quot;token=&quot;+token);
			}else{
				window.location.replace(redirecturl+&quot;&amp;token=&quot;+token);
			}
		}
	};

	//获取页面传值
	function getData(key){
		var h=decodeURIComponent(window.location.href);
		// var arr=h.substring(h.indexOf(&quot;?&quot;)+1).split(&quot;&amp;&quot;);
		// for(var i=0;i&lt;arr.length;i++){
		// 	if(arr[i].indexOf(key)>-1){
		// 		return (arr[i].split(&quot;=&quot;))[1];
		// 	}
		// }
		if(key===&quot;accesskey&quot;&amp;&amp;h.indexOf(&quot;accesskey&quot;)>-1){
			return h.substring(h.indexOf(&quot;accesskey&quot;)+(&quot;accesskey&quot;.length+1),h.indexOf(&quot;projectname&quot;)-1);
		}else if(key===&quot;projectname&quot;&amp;&amp;h.indexOf(&quot;projectname&quot;)>-1){
			return h.substring(h.indexOf(&quot;projectname&quot;)+(&quot;projectname&quot;.length+1),h.indexOf(&quot;redirecturl&quot;)-1);
		}else if(key===&quot;redirecturl&quot;&amp;&amp;h.indexOf(&quot;redirecturl&quot;)>-1){
			return h.substring(h.indexOf(&quot;redirecturl&quot;)+(&quot;redirecturl&quot;.length+1));
		}else{
			return '';
		}
	};

	//是否记住密码
	function checkRemember(){
		if($('.remember').is(&quot;:hidden&quot;)){
			// do nothing
		}else{
			if(localStorage.checker==&quot;true&quot;){
				$loginName.val(localStorage.username);
				$(&quot;#rememberCheck&quot;).prop(&quot;checked&quot;,true);
			}else{
				$loginName.val(&quot;&quot;);
				$(&quot;#rememberCheck&quot;).prop(&quot;checked&quot;,false);
			}
		}
	};

	//切换默认登录方式
	function switchLoginStyle(){
		setTimeout(function(){
			if(localStorage.loginStyleMemory&amp;&amp;projectname!=''&amp;&amp;!$(&quot;.loginStyle&quot;).is(&quot;:hidden&quot;)){
				$('.loginStyle a[flag=&quot;'+localStorage.loginStyleMemory+'&quot;]').click();
			}else{
				$('.loginStyle a[flag=&quot;wechat&quot;]').click();
			}
		},500);
	};

	//切换样式表
	function switchStyle(pn){
		if(pn==&quot;&quot;){
			$(&quot;#cssList&quot;).attr(&quot;href&quot;,&quot;style/style.css&quot;);
		}else{
			$(&quot;#cssList&quot;).attr(&quot;href&quot;,&quot;style/style_&quot;+pn+&quot;.css&quot;);
		}
	};

	//计时返回第三方重新获取accsesKey
	function goBackRedirecturl(url){
		setTimeout(function(){
			window.location.replace(url);
		},3000000);
	};

	//是否显示验证码
	function showVercode(accesskey){
		if(localStorage.accesskey==accesskey){
			if(localStorage.ers>0){
				$('#verLi').show();
			}else{
				$('#verLi').hide();
			}
		}else{
			localStorage.accesskey=accesskey;
			localStorage.ers=0;
			$('#verLi').hide();
		}
	}
	
	//先切换下样式表 和 获取一下sessionkey 和 看看有没有记住密码 和 切换下上一次的登录方式
	getSessionKey(accesskey,redirecturl); //请求sessionKey
	showVercode(accesskey);//是否显示验证码
	// $('#projectTitle').html('木哈哈哈哈标题');
})();
Capture object: Alt ` ● Load DOM Map: Ctrl Alt ` /html[1]/body[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
   </webElementXpaths>
</WebElementEntity>
