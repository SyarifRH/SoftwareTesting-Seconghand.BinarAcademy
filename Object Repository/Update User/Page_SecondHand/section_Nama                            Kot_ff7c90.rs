<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>section_Nama                            Kot_ff7c90</name>
   <tag></tag>
   <elementGuidId>cdb22ce3-74f7-4a5a-a423-979996f1133e</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>section.pt-5.mt-5</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Lengkapi Info Akun'])[1]/following::section[1]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>section</value>
      <webElementGuid>f4db68e2-52de-48f4-9eae-f72ca7e6c2b9</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>pt-5 mt-5</value>
      <webElementGuid>8ae91b84-3717-4727-b671-56cd67774a81</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
      
  
  
    
  


  
  
    

      
        
            
              
            

          
        
      

      
        Nama
        
      

      
        Kota
        Pilih Kota
Solo
Jogja
Jakarta
Bandung
Semarang
      

      
        Alamat
        Jl. Jalan II
      

      
        No Handphone
        
      

      
        
      
  



  var formAvatarView = document.getElementById('form-avatar-view')
  var formAvatarInput = document.getElementById('form-avatar-input')
  var formAvatar = document.getElementById('form-avatar')
  var formAvatarImage = document.getElementById('form-avatar-image')
  var image;

  formAvatarView.addEventListener('click', function(e) {
    formAvatar.click();
  });

  formAvatar.addEventListener('change', function () {
    if (!!formAvatarImage) {
      formAvatarImage.style[&quot;display&quot;] = &quot;none&quot;;
    }

    if (!!image) {
      image.remove();
    }

    for (const file of formAvatar.files) {
      image = document.createElement('img');
      image.classList.add(&quot;p-0&quot;, &quot;img-thumbnail&quot;, &quot;img-avatar-profile&quot;, &quot;overflow-hidden&quot;, &quot;rounded-4&quot;, &quot;border-0&quot;);
      image.src = URL.createObjectURL(file);
      formAvatarView.prepend(image);
      formAvatarInput.style['display'] = 'none';
    }
  });




    </value>
      <webElementGuid>915f4745-e932-4ee3-9dcc-d98c73aabd2e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]/section[@class=&quot;pt-5 mt-5&quot;]</value>
      <webElementGuid>e4076fa9-291f-4bd5-9c30-9f81c99bcc56</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Lengkapi Info Akun'])[1]/following::section[1]</value>
      <webElementGuid>e7f410a5-62c5-410d-a3b8-4f36f9c17768</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//section</value>
      <webElementGuid>0403c440-97f6-4b23-a119-884ee59c68c2</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//section[(text() = concat(&quot;
      
  
  
    
  


  
  
    

      
        
            
              
            

          
        
      

      
        Nama
        
      

      
        Kota
        Pilih Kota
Solo
Jogja
Jakarta
Bandung
Semarang
      

      
        Alamat
        Jl. Jalan II
      

      
        No Handphone
        
      

      
        
      
  



  var formAvatarView = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-view&quot; , &quot;'&quot; , &quot;)
  var formAvatarInput = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-input&quot; , &quot;'&quot; , &quot;)
  var formAvatar = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar&quot; , &quot;'&quot; , &quot;)
  var formAvatarImage = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-image&quot; , &quot;'&quot; , &quot;)
  var image;

  formAvatarView.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
    formAvatar.click();
  });

  formAvatar.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function () {
    if (!!formAvatarImage) {
      formAvatarImage.style[&quot;display&quot;] = &quot;none&quot;;
    }

    if (!!image) {
      image.remove();
    }

    for (const file of formAvatar.files) {
      image = document.createElement(&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;);
      image.classList.add(&quot;p-0&quot;, &quot;img-thumbnail&quot;, &quot;img-avatar-profile&quot;, &quot;overflow-hidden&quot;, &quot;rounded-4&quot;, &quot;border-0&quot;);
      image.src = URL.createObjectURL(file);
      formAvatarView.prepend(image);
      formAvatarInput.style[&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;] = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
    }
  });




    &quot;) or . = concat(&quot;
      
  
  
    
  


  
  
    

      
        
            
              
            

          
        
      

      
        Nama
        
      

      
        Kota
        Pilih Kota
Solo
Jogja
Jakarta
Bandung
Semarang
      

      
        Alamat
        Jl. Jalan II
      

      
        No Handphone
        
      

      
        
      
  



  var formAvatarView = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-view&quot; , &quot;'&quot; , &quot;)
  var formAvatarInput = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-input&quot; , &quot;'&quot; , &quot;)
  var formAvatar = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar&quot; , &quot;'&quot; , &quot;)
  var formAvatarImage = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-image&quot; , &quot;'&quot; , &quot;)
  var image;

  formAvatarView.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
    formAvatar.click();
  });

  formAvatar.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function () {
    if (!!formAvatarImage) {
      formAvatarImage.style[&quot;display&quot;] = &quot;none&quot;;
    }

    if (!!image) {
      image.remove();
    }

    for (const file of formAvatar.files) {
      image = document.createElement(&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;);
      image.classList.add(&quot;p-0&quot;, &quot;img-thumbnail&quot;, &quot;img-avatar-profile&quot;, &quot;overflow-hidden&quot;, &quot;rounded-4&quot;, &quot;border-0&quot;);
      image.src = URL.createObjectURL(file);
      formAvatarView.prepend(image);
      formAvatarInput.style[&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;] = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
    }
  });




    &quot;))]</value>
      <webElementGuid>67084602-959d-4fcd-8476-2206bd0222ff</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
